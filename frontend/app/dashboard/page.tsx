"use client";
import React, { useState } from "react";
import { useDeleteMailMutation, useGetMailsQuery } from "../services/MailApi";
import DataTable from "@/components/DataTable";
import columns from "@/app/dashboard/campaigns/[id]/analytics/_columns";

function Dashboard() {
  const [offset, setOffset] = useState(0);
  const { data: mails, isLoading } = useGetMailsQuery({ offset });
  const [deleteMail, { error: deletionError }] = useDeleteMailMutation();

  const deleteMailHandler = async (id: string) => {
    if (deletionError) {
      return <div>Error deleting the mail</div>;
    }

    await deleteMail(id);
  };

  return (
    <div>
      <h1 className="text-xl font-bold">Dashboard Overview</h1>
      <div className="mt-4">
        <DataTable
          data={mails?.items ?? []}
          columns={columns(deleteMailHandler)}
          fallback={"No Mails Found"}
          isLoading={isLoading}
          pagination={
            mails ? { page: mails, onOffsetChange: setOffset } : undefined
          }
        />
      </div>
    </div>
  );
}

export default Dashboard;
