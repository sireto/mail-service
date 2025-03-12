"use client";
import React, { useState } from "react";
import { useGetCampaignSendersQuery } from "@/app/services/CampaignSenderApi";
import DataTable from "@/components/DataTable";
import columns from "./_columns";
import { Button } from "@/components/ui/button";
import { AddCampaignSender } from "./_components/AddCampaignSender";

const CampaignSenderPage = () => {
  const [isOpen, setIsOpen] = useState(false);
  const { data: senders, error, isLoading } = useGetCampaignSendersQuery();

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <div className="p-6">
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-2xl font-semibold">
            Campaign Senders{" "}
            <span className="text-gray-500">({senders?.length || 0})</span>
          </h1>
          <Button
            variant="default"
            className="bg-blue-600 hover:bg-blue-700"
            onClick={() => setIsOpen(true)}
          >
            + New
          </Button>
        </div>
      </div>

      <AddCampaignSender open={isOpen} onClose={() => setIsOpen(false)} />

      <div className="p-6">
        <DataTable
          data={senders || []}
          columns={columns}
          fallback="No campaign senders found"
        />
      </div>
    </div>
  );
};

export default CampaignSenderPage;
