"use client";

import React from "react";

import DataTable from "@/components/DataTable";
import columns from "./_columns";
import {
  useDeleteCampaignMutation,
  useGetCampaignsQuery,
  useStartCampaignMutation,
} from "@/app/services/CampaignApi";
import Link from "next/link";
import AddButton from "@/components/common/AddButton";

export default function CampaignPage() {
  const { data: campaigns, error, isLoading } = useGetCampaignsQuery();
  const [deleteCampaign, { error: deletionError }] =
    useDeleteCampaignMutation();
  const [startCampaign, { error: startingError }] = useStartCampaignMutation();

  if (error) {
    return <div>There was an error fetching campaigns...</div>;
  }

  if (isLoading) {
    return <div> Loading... </div>;
  }

  const deleteCampaignHandler = async (id: string) => {
    try {
      if (deletionError) {
        // if there is an error already why bother sending request...
        console.error("Error deleting the campaign");
        return;
      }

      await deleteCampaign(id);
    } catch (error) {
      console.error("Failed to delete campaign:", error);
    }
  };

  const startCampaignHandler = async (id: string) => {
    try {
      if (startingError) {
        console.error("Error starting the campaign: ", deletionError);
        return;
      }

      const campaignData = {
        campaignId: id,
        listId: "0a48a82f-ec04-4c19-904c-48dcebc80e49",
      };

      await startCampaign(campaignData);
      window.alert("Campaign started successfully");
    } catch (error) {
      console.error("Failed to start the campaign: ", error);
      window.alert("Failed to start the campaign. Please try again.");
    }
  };

  return (
    <div className="">
      {/* Template CampaignsPage heading... */}
      <div className="w-full flex justify-between items-center">
        <h1 className="text-xl font-bold">
          Campaigns
          <span>({campaigns?.length})</span>
        </h1>
        <Link href={"/dashboard/campaigns/new"}>
          <AddButton />
        </Link>
      </div>
      <div className="my-12">
        <DataTable
          data={campaigns || []}
          columns={columns(deleteCampaignHandler, startCampaignHandler)}
          fallback={"No campaigns found"}
        />
      </div>
    </div>
  );
}
