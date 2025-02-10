"use client";
import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import ServerCard from "@/components/ServerCard";
import type { Server } from "@/lib/type";

export default function ServerPage() {
  const [servers, setServers] = useState<Server[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    fetchServers();
  }, []);

  const fetchServers = async () => {
    try {
      setIsLoading(true);
      const response = await fetch("http://localhost:8000/api/servers");
      const data = await response.json();
      setServers(data);
    } catch (error) {
      console.error("Error fetching servers:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleAddServer = () => {
    const newServer: Partial<Server> = {
      namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
      port: 587,
      tls_type: "NONE",
    };
    setServers([...servers, newServer as Server]);
  };

  const handleUpdateServer = async (
    serverId: string,
    updatedServer: Server
  ) => {
    try {
      const response = await fetch(
        `http://localhost:8000/api/servers/${serverId}`,
        {
          method: "PATCH",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(updatedServer),
        }
      );

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Failed to update server: ${errorText}`);
      }

      setServers(
        servers.map((server) =>
          server.id === serverId ? { ...server, ...updatedServer } : server
        )
      );
    } catch (error) {
      console.error("Error updating server:", error);
      throw error;
    }
  };

  const handleDeleteServer = async (serverId: string) => {
    try {
      const response = await fetch(
        `http://localhost:8000/api/servers/${serverId}`,
        {
          method: "DELETE",
        }
      );

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Failed to delete server: ${errorText}`);
      }

      setServers(servers.filter((server) => server.id !== serverId));
    } catch (error) {
      console.error("Error deleting server:", error);
      throw error;
    }
  };

  const handleCreateServer = async (newServer: Omit<Server, "id">) => {
    try {
      console.log("Creating server with payload:", newServer);

      const response = await fetch("http://localhost:8000/api/servers", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newServer),
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Failed to create server: ${errorText}`);
      }

      const createdServer = await response.json();

      setServers((currentServers) =>
        currentServers.map((server) => (!server.id ? createdServer : server))
      );

      return createdServer;
    } catch (error) {
      console.error("Error creating server:", error);
      throw error;
    }
  };

  if (isLoading) return <div>Loading...</div>;

  return (
    <>
      <h1>Server Settings</h1>
      <div className="flex justify-center">
        <div className="space-y-4">
          {servers.map((server) => (
            <ServerCard
              key={server.id || "new"}
              server={server}
              onUpdate={handleUpdateServer}
              onDelete={handleDeleteServer}
              onCreate={handleCreateServer}
            />
          ))}
          <div className="mt-4">
            <Button onClick={handleAddServer}>Add New Server</Button>
          </div>
        </div>
      </div>
    </>
  );
}
