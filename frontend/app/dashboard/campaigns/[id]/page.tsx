"use client";

import { AddCampaignFormSchemaDTO } from "@/lib/type";
import React, { useState, useEffect } from "react";
import { useParams, useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  useCreateCampaignMutation,
  useGetCampaignByIdQuery,
  useUpdateCampaignMutation,
} from "@/app/services/CampaignApi";
import { Save } from "lucide-react";
import { Button } from "@/components/ui/button";
import CampaignForm from "./_components/CampaignForm";
import SendTestMailForm from "./_components/SendTestMailForm";
import { useLazyGetTemplatesQuery } from "@/app/services/TemplateApi";
import { useGetCampaignSendersQuery } from "@/app/services/CampaignSenderApi";
import { NAMESPACE_ID } from "@/config/namespace";
import { MAX_PAGE_SIZE } from "@/lib/type/pagination";
import type { Template } from "@/lib/type/template";

const Page = () => {
  const { id } = useParams<{ id: string }>();
  const router = useRouter();
  const isEditing = id !== "new"; // if not new this Page is opened in the editing mode...
  const form = useForm<z.infer<typeof AddCampaignFormSchemaDTO>>({
    resolver: zodResolver(AddCampaignFormSchemaDTO),
    defaultValues: {
      campaign_name: "",
      campaign_senders: "",
      namespace_id: "",
      template_id: "",
      list_ids: [],
    },
  });

  const { data: campaignData } = useGetCampaignByIdQuery(id, {
    skip: !isEditing,
  });
  const [createCampaign] = useCreateCampaignMutation();
  const [updateCampaign] = useUpdateCampaignMutation();

  const [templates, setTemplates] = useState<Template[]>([]);
  const [isTemplateLoading, setIsTemplateLoading] = useState(true);
  const [templateLoadError, setTemplateLoadError] = useState(false);
  const [fetchTemplatesPage] = useLazyGetTemplatesQuery();

  const { data: campaignSenders, isLoading: isSenderLoading } =
    useGetCampaignSendersQuery(undefined, { refetchOnMountOrArgChange: true });

  useEffect(() => {
    let cancelled = false;

    const loadTemplates = async () => {
      setIsTemplateLoading(true);
      setTemplateLoadError(false);

      try {
        const allTemplates: Template[] = [];
        let offset = 0;

        for (;;) {
          const page = await fetchTemplatesPage({
            limit: MAX_PAGE_SIZE,
            offset,
          }).unwrap();

          if (cancelled) return;

          allTemplates.push(...page.items);
          offset += page.items.length;

          if (
            !page.has_more ||
            page.items.length === 0 ||
            allTemplates.length >= page.total
          ) {
            break;
          }
        }

        if (!cancelled) setTemplates(allTemplates);
      } catch {
        if (!cancelled) setTemplateLoadError(true);
      } finally {
        if (!cancelled) setIsTemplateLoading(false);
      }
    };

    void loadTemplates();

    return () => {
      cancelled = true;
    };
  }, [fetchTemplatesPage]);

  useEffect(() => {
    if (
      isEditing &&
      campaignData &&
      !isTemplateLoading &&
      !isSenderLoading &&
      templates &&
      campaignSenders
    ) {
      form.reset({
        campaign_name: campaignData.campaign_name,
        campaign_senders: campaignData.campaign_senders,
        namespace_id: campaignData.namespace_id,
        template_id: campaignData.template_id,
        list_ids: campaignData.lists.map((list) => list.id),
      });
    }
  }, [
    campaignData,
    isEditing,
    isTemplateLoading,
    isSenderLoading,
    form,
    campaignSenders,
    templates,
  ]);

  const saveCampaignChanges = async (
    value: z.infer<typeof AddCampaignFormSchemaDTO>,
  ) => {
    if (isEditing) {
      const updatedCampaign = {
        campaign_name: value.campaign_name.trim(),
        campaign_senders: value.campaign_senders.trim(),
        status: "draft", // Update status if needed
        template_id: value.template_id.trim(),
        scheduled_at: "2023-01-01T00:00:00Z", // Ensure this is in the correct format,
        list_ids: value.list_ids,
      };

      const updatedCampaignData = {
        campaignId: id,
        updatedCampaign: updatedCampaign,
      };

      await updateCampaign(updatedCampaignData);
      form.reset();
    } else {
      const newCampaign = {
        campaign_name: value.campaign_name.trim(),
        campaign_senders: value.campaign_senders.trim(),
        namespace_id: NAMESPACE_ID,
        template_id: value.template_id.trim(),
        status: "draft",
        list_ids: value.list_ids,
        // scheduled_at: (new Date()).toISOString(),
        scheduled_at: "2025-02-10T12:00:00",
      };

      await createCampaign(newCampaign);
      form.reset();
    }
    router.push("/dashboard/campaigns");
  };

  if (templateLoadError) {
    return <div>There was an error fetching templates...</div>;
  }

  return (
    <div className="flex flex-col space-y-8 lg:space-y-0 lg:flex-row lg:justify-between lg:gap-x-64">
      <CampaignForm
        form={form}
        submitHandler={saveCampaignChanges}
        triggerButton={
          <Button
            type="submit"
            className="bg-primary text-white py-2 px-4 rounded-md"
          >
            <Save size={16} />
            <span>Save Changes</span>
          </Button>
        }
        templates={templates}
        senders={campaignSenders}
      />
      <hr className="lg:hidden" />
      <SendTestMailForm templateId={form.getValues("template_id")} />
    </div>
  );
};

export default Page;
