import { useGetCampaignByIdQuery } from "@/app/services/CampaignApi";
import React from "react";

const CampaignName = ({ id }: { id: string }) => {
  const { data: campaign, error, isLoading } = useGetCampaignByIdQuery(id);

  if (error) {
    return <span>There was an error fetching the campaign...</span>;
  }

  if (isLoading) {
    return <span>Loading...</span>;
  }

  return <span>{campaign?.campaign_name}</span>;
};

export default CampaignName;
