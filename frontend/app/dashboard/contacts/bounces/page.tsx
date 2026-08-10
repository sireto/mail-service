"use client";

import React, { Suspense, useState } from "react";
import { Button } from "@/components/ui/button";
import { Trash2 } from "lucide-react";
import DataTable from "@/components/DataTable";
import columns from "./_columns";
import {
  useDeleteMailMutation,
  useGetBouncedMailQuery,
} from "@/app/services/MailApi";
import ConfirmationPopup from "@/components/common/ConfirmationPopup";

const Page = () => {
  const { data: bouncedMails } = useGetBouncedMailQuery();
  const [deleteMail] = useDeleteMailMutation();

  const [selectedBounces, setSelectedBounces] = useState<
    Record<string, boolean>
  >({});

  const deleteMailHandler = async (id: string) => {
    await deleteMail(id).unwrap();
  };

  const handleBulkMailDelete = async () => {
    const bounceIds = Object.keys(selectedBounces).filter(
      (id) => selectedBounces[id],
    );

    if (bounceIds.length === 0) return;

    try {
      await Promise.all(bounceIds.map((id) => deleteMail(id).unwrap()));
      setSelectedBounces({});
    } catch (error) {
      console.error("Failed to delete selected bounces:", error);
    }
  };

  return (
    <Suspense fallback={<div>Loading bounces...</div>}>
      <div>
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-2xl font-semibold">
            Bounces{" "}
            <span className="text-gray-500">({bouncedMails?.length || 0})</span>
          </h1>
          {Object.keys(selectedBounces).length !== 0 && (
            <ConfirmationPopup
              title="Are you sure to delete selected bounced mails?"
              message="The selected bounced mails will be deleted permanently."
              onConfirm={handleBulkMailDelete}
              popupTriggerButton={
                <Button
                  variant="ghost"
                  size="sm"
                  className="flex items-center gap-2 text-red-600 hover:text-red-700"
                >
                  <Trash2
                    strokeWidth={1.5}
                    size={20}
                    className="text-red-400 hover:text-red-500 transition-all duration-300 ease-in-out hover:scale-105 cursor-pointer"
                  />
                  <span>Delete</span>
                </Button>
              }
              confirmButton={<Button variant={"danger"}>Delete</Button>}
            />
          )}
        </div>
        {/* DataTable */}
        <div className="py-6 mb-6">
          <DataTable
            data={bouncedMails || []}
            columns={columns(
              selectedBounces,
              setSelectedBounces,
              deleteMailHandler,
            )}
            fallback="No Bounce mails Found"
          />
        </div>
      </div>
    </Suspense>
  );
};

export default Page;
