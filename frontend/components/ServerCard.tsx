"use client";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { Minus, Plus, TestTube2 } from "lucide-react";

export default function ServerCard() {
  // Directly manage port value in state
  const [port, setPort] = useState(25);

  // Handle port increment and decrement
  const handlePortChange = (action: "increase" | "decrease") => {
    setPort((prevPort) =>
      action === "increase" ? prevPort + 1 : Math.max(prevPort - 1, 0)
    );
  };

  // Handle delete action
  const handleDelete = () => {
    if (confirm("Are you sure you want to delete this configuration?")) {
      console.log("Deleted the SMTP configuration");
    }
  };

  return (
    <Card className="w-full max-w-4xl mx-auto">
      <CardContent className="p-6 space-y-6">
        {/* Enabled Switch */}
        <div className="flex items-center space-x-4">
          <Switch id="enabled" />
          <Label htmlFor="enabled">Enabled</Label>
          <Button
            variant="ghost"
            className="text-blue-600 hover:text-blue-800 ml-auto"
            onClick={handleDelete}
          >
            Delete
          </Button>
        </div>

        {/* Host and Port */}
        <div className="grid md:grid-cols-[2fr,1fr] gap-4">
          <div className="space-y-2">
            <Label htmlFor="host">Host</Label>
            <Input id="host" placeholder="smtp.yoursite.com" />
            <p className="text-sm text-muted-foreground">
              SMTP server&apos;s host address.
            </p>
          </div>
          <div className="space-y-2">
            <Label htmlFor="port">Port</Label>
            <div className="flex space-x-2">
              <Button
                variant="outline"
                size="icon"
                onClick={() => handlePortChange("decrease")}
              >
                <Minus className="h-4 w-4" />
              </Button>
              <Input
                id="port"
                value={port}
                onChange={(e) => setPort(Number(e.target.value))}
                className="text-center"
              />
              <Button
                variant="outline"
                size="icon"
                onClick={() => handlePortChange("increase")}
              >
                <Plus className="h-4 w-4" />
              </Button>
            </div>
            <p className="text-sm text-muted-foreground">
              SMTP server&apos;s port.
            </p>
          </div>
        </div>

        {/* Auth Protocol, Username, Password */}
        <div className="grid md:grid-cols-[2fr,2fr] gap-4">
          <div className="space-y-2">
            <Label htmlFor="username">User Name</Label>
            <Input id="username" placeholder="username" />
          </div>
          <div className="space-y-2">
            <Label htmlFor="password">Password</Label>
            <Input id="password" type="password" />
            <p className="text-sm text-muted-foreground">Enter to change</p>
          </div>
        </div>

        {/* HELO and TLS */}
        <div className="grid md:grid-cols-[2fr,1fr,1fr] gap-4">
          <div className="space-y-2">
            <Label>TLS</Label>
            <Select defaultValue="STARTTLS">
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="STARTTLS">STARTTLS</SelectItem>
                <SelectItem value="TLS">TLS</SelectItem>
                <SelectItem value="NONE">NONE</SelectItem>
              </SelectContent>
            </Select>
            <p className="text-sm text-muted-foreground">
              TLS/SSL encryption. STARTTLS is commonly used.
            </p>
          </div>
        </div>

        {/* Custom Headers */}
        <div>
          <Button variant="outline" className="w-full justify-start">
            + Set custom headers
          </Button>
        </div>

        {/* Test Connection */}
        <div className="flex justify-end">
          <Button>
            <TestTube2 className="mr-2 h-4 w-4" />
            Test connection
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
