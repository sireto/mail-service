"use client";
import { useGetServersQuery } from "@/app/services/ServerApi";
import ServerCard from "@/components/ServerCard";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import NoServersFound from "@/app/dashboard/servers/NotFound";

export default function ServerPage() {
  const { data: servers, isLoading, isError } = useGetServersQuery();
  const [showNewServerCard, setShowNewServerCard] = useState(false);

  if (isLoading) return <div>Loading...</div>;
  if (isError) return <div>Error loading servers</div>;

  if (servers?.length === 0) return <NoServersFound />;

  return (
    <>
      <h1>Server Settings</h1>
      <div className="flex justify-center">
        <div className="space-y-4">
          {servers?.map((server) => (
            <ServerCard key={server.id} server={server} />
          ))}
          {showNewServerCard && (
            <ServerCard
              key="new"
              server={{}}
              onCancel={() => setShowNewServerCard(false)}
            />
          )}
          {!showNewServerCard && (
            <Button onClick={() => setShowNewServerCard(true)}>
              Add New Server
            </Button>
          )}
        </div>
      </div>
    </>
  );
}
