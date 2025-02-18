"use client";

import React, { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Switch } from "@/components/ui/switch";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Upload } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useImportContactsMutation } from "@/app/services/ContactApi";
import { useToast } from "@/hooks/use-toast";

export default function ImportPage() {
  const [mode, setMode] = useState("subscribe");
  const [status, setStatus] = useState("unconfirmed");
  const [overwrite, setOverwrite] = useState(false);
  const [file, setFile] = useState<File | null>(null);
  const [delimiter, setDelimiter] = useState(",");
  const [selectedLists, setSelectedLists] = useState<string[]>([]);
  const [isUploading, setIsUploading] = useState(false);

  const [importContacts, { isLoading }] = useImportContactsMutation();
  const { toast } = useToast();

  const handleFileDrop = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    const droppedFile = e.dataTransfer.files[0];
    if (
      droppedFile?.type === "text/csv" ||
      droppedFile?.name.endsWith(".csv") ||
      droppedFile?.name.endsWith(".zip")
    ) {
      setFile(droppedFile);
    } else {
      toast({
        title: "Invalid file type",
        description: "Please upload a CSV or ZIP file.",
        variant: "destructive",
      });
    }
  };

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0];
    if (selectedFile) {
      if (
        selectedFile.type === "text/csv" ||
        selectedFile.name.endsWith(".csv") ||
        selectedFile.name.endsWith(".zip")
      ) {
        setFile(selectedFile);
      } else {
        toast({
          title: "Invalid file type",
          description: "Please upload a CSV or ZIP file.",
          variant: "destructive",
        });
      }
    }
  };

  const handleUpload = async () => {
    if (!file) {
      toast({
        title: "No file selected",
        description: "Please select a CSV or ZIP file to upload.",
        variant: "destructive",
      });
      return;
    }

    try {
      setIsUploading(true);
      const formData = new FormData();
      formData.append("file", file);
      formData.append("mode", mode);
      formData.append("status", status);
      formData.append("overwrite", overwrite.toString());
      formData.append("delimiter", delimiter);
      formData.append("lists", JSON.stringify(selectedLists));

      const result = await importContacts(formData).unwrap();

      toast({
        title: "Import successful",
        description: `Successfully imported ${result.imported} contacts.`,
        variant: "default",
      });

      // Reset the form
      setFile(null);
    } catch (error: any) {
      toast({
        title: "Import failed",
        description:
          error.data?.message || "Something went wrong. Please try again.",
        variant: "destructive",
      });
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div className="container p-6 max-w-6xl">
      <Card>
        <CardHeader>
          <CardTitle>Import subscribers</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="flex gap-8">
            {/* Mode Selection */}
            <div className="space-y-3">
              <Label>Mode</Label>
              <RadioGroup
                defaultValue={mode}
                onValueChange={setMode}
                className="flex space-x-4"
              >
                <div className="flex items-center space-x-2">
                  <RadioGroupItem value="subscribe" id="subscribe" />
                  <Label htmlFor="subscribe">Subscribe</Label>
                </div>
                <div className="flex items-center space-x-2">
                  <RadioGroupItem value="blocklist" id="blocklist" />
                  <Label htmlFor="blocklist">Blocklist</Label>
                </div>
              </RadioGroup>
            </div>

            {/* Status Selection */}
            <div className="space-y-3">
              <Label>Status</Label>
              <RadioGroup
                defaultValue={status}
                onValueChange={setStatus}
                className="flex space-x-4"
              >
                <div className="flex items-center space-x-2">
                  <RadioGroupItem value="unconfirmed" id="unconfirmed" />
                  <Label htmlFor="unconfirmed">Unconfirmed</Label>
                </div>
                <div className="flex items-center space-x-2">
                  <RadioGroupItem value="confirmed" id="confirmed" />
                  <Label htmlFor="confirmed">Confirmed</Label>
                </div>
              </RadioGroup>
            </div>

            {/* Overwrite Switch */}
            <div className="flex flex-col justify-between">
              <Label>Overwrite?</Label>
              <Switch checked={overwrite} onCheckedChange={setOverwrite} />
              <div className="space-y-0.5">
                <div className="text-sm text-muted-foreground">
                  Overwrite name, attributes, subscription status of existing
                  subscribers?
                </div>
              </div>
            </div>

            {/* CSV Delimiter */}
            <div className="space-y-2">
              <Label>CSV delimiter</Label>
              <Input
                value={delimiter}
                onChange={(e) => setDelimiter(e.target.value)}
                className="w-20"
                placeholder=","
              />
              <p className="text-sm text-muted-foreground">
                Default delimiter is comma.
              </p>
            </div>
          </div>

          {/* Lists Selection */}
          <div className="space-y-2">
            <Label>Lists</Label>
            <Select
              onValueChange={(value) =>
                setSelectedLists([...selectedLists, value])
              }
            >
              <SelectTrigger className="w-full">
                <SelectValue placeholder="Lists to subscribe to..." />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="list1">List 1</SelectItem>
                <SelectItem value="list2">List 2</SelectItem>
              </SelectContent>
            </Select>
            {selectedLists.length > 0 && (
              <div className="mt-2 flex flex-wrap gap-2">
                {selectedLists.map((list) => (
                  <div
                    key={list}
                    className="bg-gray-100 px-2 py-1 rounded-md flex items-center gap-1"
                  >
                    {list}
                    <button
                      onClick={() =>
                        setSelectedLists(
                          selectedLists.filter((l) => l !== list)
                        )
                      }
                      className="text-gray-500 hover:text-gray-700"
                    >
                      ×
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* File Upload Area */}
          <div
            className={`border-2 border-dashed rounded-lg p-8 text-center cursor-pointer hover:bg-green-100 opacity-50 ${
              isUploading ? "opacity-50 pointer-events-none" : ""
            }`}
            onDragOver={(e) => e.preventDefault()}
            onDrop={handleFileDrop}
            onClick={() => document.getElementById("file-upload")?.click()}
          >
            <input
              id="file-upload"
              type="file"
              className="hidden"
              accept=".csv,.zip"
              onChange={handleFileSelect}
              disabled={isUploading}
            />
            <Upload className="mx-auto h-12 w-12 text-gray-400" />
            <div className="mt-4">
              {file ? (
                <p className="text-sm font-medium">{file.name}</p>
              ) : (
                <p className="text-sm text-gray-500">
                  Click or drag a CSV or ZIP file here
                </p>
              )}
            </div>
          </div>

          {/* Instructions */}
          <div>
            <p>
              Upload a CSV file or a ZIP file with a single CSV file in it to
              bulk import subscribers. The CSV file should have the following
              headers with the exact column names:
              <code className="ml-1 text-sm bg-gray-100 px-1 rounded">
                email, name, attributes
              </code>
              <br />
              Attributes (optional) should be a valid JSON string with double
              escaped quotes.
            </p>
          </div>

          {/* Example */}
          <div className="space-y-2">
            <Label>Example raw CSV</Label>
            <pre className="bg-gray-100 p-4 rounded-lg text-sm overflow-x-auto">
              {`email, name, attributes
saisab@gmail.com, "Saisab", "{\\"age\\": 20, \\"color\\": \\"red\\"}"
erlich@gmail.com, "Erlich", "{\\"age\\": 24, \\"job\\": \\"Professor\\"}"`}
            </pre>
          </div>

          {/* Upload Button */}
          <Button
            className="w-full"
            size="lg"
            onClick={handleUpload}
            disabled={isUploading || !file}
          >
            {isUploading ? "Uploading..." : "Upload"}
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
