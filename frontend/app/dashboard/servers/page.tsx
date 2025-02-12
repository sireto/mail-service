"use client";
import { useGetServersQuery } from "@/app/dashboard/servers/serverApi";
import ServerCard from "@/components/ServerCard";
import { Button } from "@/components/ui/button";
import { useState } from "react";

export default function ServerPage() {
  const { data: servers, isLoading, isError } = useGetServersQuery();
  const [showNewServerCard, setShowNewServerCard] = useState(false);

  if (isLoading) return <div>Loading...</div>;
  if (isError) return <div>Error loading servers</div>;

  return (
    <>
      <h1>Server Settings</h1>
      <div className="flex justify-center">
        <div className="space-y-4">
          {servers?.map((server) => (
            <ServerCard key={server.id} server={server} />
          ))}
          {showNewServerCard && <ServerCard key="new" server={{}} />}
          <Button onClick={() => setShowNewServerCard(true)}>
            Add New Server
          </Button>
        </div>
      </div>
    </>
  );
}
