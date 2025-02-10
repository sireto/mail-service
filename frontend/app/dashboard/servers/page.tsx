"use client";
import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import ServerCard from "@/components/ServerCard";
import { Server } from "@/lib/schema";

export default function ServerPage() {
  const [servers, setServers] = useState<Server[]>([]);

  // Fetch initial servers
  useEffect(() => {
    const fetchServers = async () => {
      try {
        const response = await fetch("http://localhost:8000/api/servers");
        const data = await response.json();
        setServers(data);
      } catch (error) {
        console.error("Error fetching servers:", error);
      }
    };
    fetchServers();
  }, []);

  return (
    <div className="space-y-4">
      <div className="flex gap-4">
        <Button>Add New Server</Button>
        <Button>Save Changes</Button>
      </div>

      <div className="space-y-4">
        {servers.map((server) => (
          <ServerCard key={server.created_at} />
        ))}
      </div>
    </div>
  );
}
