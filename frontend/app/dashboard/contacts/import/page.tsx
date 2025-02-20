"use client";

import React, { useState, useRef } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Switch } from "@/components/ui/switch";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Upload } from "lucide-react";
import { MultiSelect } from "@/components/multi-select";
import { useImportContactsMutation } from "@/app/services/ContactApi";
import { useGetListsQuery } from "@/app/services/ListApi";
import { useToast } from "@/hooks/use-toast";

// Hardcoded for now will be replaced later
const NAMESPACE_ID = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";

// will import from ListType after merge
interface List {
  id: string;
  name: string;
  description: string;
  namespace_id: string;
  created_at: string;
  updated_at: string;
}

type ImportResponse = {
  imported: number;
};

export default function ImportPage() {
  const [mode, setMode] = useState("subscribe");
  const [status, setStatus] = useState("unconfirmed");
  const [overwrite, setOverwrite] = useState(false);
  const [file, setFile] = useState<File | null>(null);
  const [delimiter, setDelimiter] = useState(",");
  const [selectedLists, setSelectedLists] = useState<string[]>([]);
  const [isUploading, setIsUploading] = useState(false);
  const multiSelectRef = useRef(null);

  const [importContacts] = useImportContactsMutation();
  const { toast } = useToast();

  const { data: listsData, isLoading: listsLoading } =
    useGetListsQuery(NAMESPACE_ID);
  const lists: List[] = listsData ?? [];

  // Transform lists data to the format expected by MultiSelect
  const listOptions = lists.map((list) => ({
    label: list.name,
    value: list.id,
  }));

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

  const handleListChange = (selectedLists: string[]) => {
    setSelectedLists(selectedLists);
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

      // Only append lists if there are any selected
      if (selectedLists.length > 0) {
        formData.append("lists", JSON.stringify(selectedLists));
      }

      const result = (await importContacts(
        formData
      ).unwrap()) as ImportResponse;

      toast({
        title: "Import successful",
        description: `Successfully imported ${result.imported} contacts.`,
        variant: "default",
      });

      // Reset the form
      setFile(null);
    } catch (error) {
      const typedError = error as { data?: { message?: string } };
      toast({
        title: "Import failed",
        description:
          typedError.data?.message || "Something went wrong. Please try again.",
        variant: "destructive",
      });
      console.error("Import error:", error);
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div className="container p-4 sm:p-6 max-w-6xl ">
      <Card>
        <CardHeader>
          <CardTitle>Import subscribers</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
            {/* Mode Selection */}
            <div className="space-y-2">
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
            <div className="space-y-2">
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
              <div className="flex items-center space-x-2 mt-2">
                <Switch checked={overwrite} onCheckedChange={setOverwrite} />
                <span className="text-sm text-muted-foreground">
                  Overwrite existing subscribers
                </span>
              </div>
              <div className="text-xs text-muted-foreground mt-1">
                Updates name, attributes, subscription status
              </div>
            </div>

            {/* CSV Delimiter */}
            <div className="space-y-2">
              <Label>CSV delimiter</Label>
              <Input
                value={delimiter}
                onChange={(e) => setDelimiter(e.target.value)}
                className="w-full sm:w-20"
                placeholder=","
              />
              <p className="text-xs text-muted-foreground">
                Default delimiter is comma.
              </p>
            </div>
          </div>

          {/* Lists Selection using MultiSelect */}
          <div className="space-y-2">
            <Label>Lists</Label>
            {listsLoading ? (
              <div className="text-sm text-muted-foreground">
                Loading lists...
              </div>
            ) : (
              <MultiSelect
                ref={multiSelectRef}
                options={listOptions}
                onValueChange={handleListChange}
                defaultValue={selectedLists}
                placeholder="Select lists to subscribe to..."
                variant="default"
                className="w-full"
                maxCount={listOptions.length}
              />
            )}
            <p className="text-xs text-muted-foreground">
              Select the lists where imported contacts will be added
            </p>
          </div>

          {/* File Upload Area */}
          <div
            className={`border-2 border-dashed rounded-lg p-6 sm:p-8 text-center cursor-pointer transition-colors hover:bg-green-50 ${
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
            <Upload className="mx-auto h-10 w-10 sm:h-12 sm:w-12 text-gray-400" />
            <div className="mt-4">
              {file ? (
                <div>
                  <p className="text-sm font-medium">{file.name}</p>
                  <p className="text-xs text-gray-500 mt-1">
                    {(file.size / 1024).toFixed(2)} KB
                  </p>
                </div>
              ) : (
                <div>
                  <p className="text-sm font-medium">
                    Click or drag a file here
                  </p>
                  <p className="text-xs text-gray-500 mt-1">
                    Accept CSV or ZIP files
                  </p>
                </div>
              )}
            </div>
          </div>

          {/* Instructions */}
          <div className="text-sm">
            <h3 className="font-medium mb-2">File Format Instructions</h3>
            <p>
              Upload a CSV file or a ZIP file with a single CSV file in it to
              bulk import subscribers. The CSV file should have the following
              headers with the exact column names:
              <code className="ml-1 text-sm bg-gray-100 px-1 rounded">
                email, name, attributes
              </code>
            </p>
            <p className="mt-2 text-xs text-gray-600">
              Attributes (optional) should be a valid JSON string with double
              escaped quotes.
            </p>
          </div>

          {/* Example */}
          <div className="space-y-2">
            <Label>Example raw CSV</Label>
            <div className="bg-gray-100 p-3 sm:p-4 rounded-lg overflow-x-auto">
              <pre className="text-xs sm:text-sm whitespace-pre-wrap">
                {`email, name, attributes
saisab@gmail.com, "Saisab", "{\\"age\\": 20, \\"color\\": \\"red\\"}"
erlich@gmail.com, "Erlich", "{\\"age\\": 24, \\"job\\": \\"Professor\\"}"`}
              </pre>
            </div>
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
