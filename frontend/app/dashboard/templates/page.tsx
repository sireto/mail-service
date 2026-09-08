"use client";

import React, { useState } from "react";

import Modal from "@/components/Modal";
import AddTemplateForm from "./_components/templateForms/AddTemplateForm";
import { useGetTemplatesQuery } from "@/app/services/TemplateApi";
import DataTable from "@/components/DataTable";
import columns from "./_columns";
import AddButton from "@/components/common/AddButton";

const Page = () => {
  const [offset, setOffset] = useState(0);
  const {
    data: templates,
    error,
    isLoading,
  } = useGetTemplatesQuery({ offset });

  if (error) {
    return <div>There was an error fetching templates...</div>;
  }

  if (isLoading) {
    return <div> Template data is loading... </div>;
  }

  return (
    <div className="">
      {/* Template Page heading... */}
      <div className="w-full flex justify-between items-center">
        <h1 className="text-xl font-bold">
          Templates
          <span>({templates?.total ?? 0})</span>
        </h1>
        <Modal
          triggerButton={<AddButton />}
          dialogBody={<AddTemplateForm />}
          dialogTitle={"New template"}
          dialogDescription={"Add a new template"}
        />
      </div>
      <div className="my-12">
        <DataTable
          data={templates?.items ?? []}
          columns={columns}
          fallback={"No templates found"}
          isLoading={isLoading}
          pagination={
            templates
              ? { page: templates, onOffsetChange: setOffset }
              : undefined
          }
        />
      </div>
    </div>
  );
};

export default Page;
