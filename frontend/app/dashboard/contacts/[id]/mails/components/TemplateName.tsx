"use client";

import React from "react";
import { useGetTemplateByIdQuery } from "@/app/services/TemplateApi";

interface TemplateNameProps {
  id: string;
}

const TemplateName = ({ id }: TemplateNameProps) => {
  const { data: template, error, isLoading } = useGetTemplateByIdQuery(id);

  if (error) {
    return <span>There was an error fetching the template...</span>;
  }

  if (isLoading) {
    return <span>Loading...</span>;
  }

  return <span>{template?.name}</span>;
};

export default TemplateName;
