"use client";
import React from "react";
import { useDeleteMailMutation, useGetMailsQuery } from "../services/MailApi";
import DataTable from "@/components/DataTable";
import columns from "@/app/dashboard/campaigns/[id]/analytics/_columns";

function Dashboard() {
  const { data: mails, isLoading, error } = useGetMailsQuery({});
  const [deleteMail, { isLoading: isDeleting, error: deletionError }] =
    useDeleteMailMutation();

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
          data={mails || []}
          columns={columns(deleteMailHandler)}
          fallback={"No Mails Found"}
        />
      </div>
    </div>
  );
}

export default Dashboard;
