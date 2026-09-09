"use client";

import React, { useState, useRef } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { FileDown } from "lucide-react";
import { MultiSelect } from "@/components/multi-select";
import { useLazyGetContactsQuery } from "@/app/services/ContactApi";
import { MAX_PAGE_SIZE } from "@/lib/type/pagination";
import {
  useGetContactsFromListsQuery,
  useGetListsQuery,
} from "@/app/services/ListApi";
import { useToast } from "@/hooks/use-toast";
import { Contact } from "@/lib/type/contact";
import { List } from "@/lib/type";
import { NAMESPACE_ID } from "@/config/namespace";

// Hardcoded for now will be replaced later


export default function ExportPage() {
  const [includeAttributes, setIncludeAttributes] = useState(true);
  const [delimiter, setDelimiter] = useState(",");
  const [selectedLists, setSelectedLists] = useState<string[]>([]);
  const [isExporting, setIsExporting] = useState(false);
  const multiSelectRef = useRef(null);

  // Export needs every contact, and the list endpoint is paginated now, so it walks the
  // pages on demand instead of holding the whole table in the page. Rendering an export
  // form does not need the rows at all.
  const [fetchContactsPage] = useLazyGetContactsQuery();
  const { toast } = useToast();

  const fetchAllContacts = async (): Promise<Contact[]> => {
    const all: Contact[] = [];
    let offset = 0;

    // Bounded by `total` rather than by trusting has_more alone, so a server that always
    // reports more cannot spin this loop forever.
    for (;;) {
      const page = await fetchContactsPage({
        limit: MAX_PAGE_SIZE,
        offset,
      }).unwrap();

      all.push(...page.items);
      offset += page.items.length;

      if (!page.has_more || page.items.length === 0 || all.length >= page.total) {
        break;
      }
    }

    return all;
  };

  const { data: listsData, isLoading: listsLoading } =
    useGetListsQuery(NAMESPACE_ID);
  const lists: List[] = listsData ?? [];

  // Transform lists data to the format expected by MultiSelect
  const listOptions = lists.map((list) => ({
    label: list.name,
    value: list.id,
  }));

  const handleListChange = (selectedLists: string[]) => {
    setSelectedLists(selectedLists);
  };

  const { data: contactsFromLists } = useGetContactsFromListsQuery(
    selectedLists,
    {
      skip: selectedLists.length === 0,
    },
  );
  console.log("Selected lists: ", selectedLists);
  console.log("contacts from Lists: ", contactsFromLists);
  const resolveContactsToExport = async (): Promise<Contact[]> => {
    if (selectedLists.length > 0 && contactsFromLists) {
      return contactsFromLists;
    }

    return fetchAllContacts();
  };

  const formatContactsAsCSV = (filteredContacts: Contact[]): string => {
    // Define CSV headers
    const headers = ["email", "first_name", "last_name", "created", "updated"];
    if (includeAttributes) {
      headers.push("attributes");
    }
    headers.push("lists");

    // Convert contacts to CSV rows
    const rows = filteredContacts.map((contact) => {
      const row = [
        contact.email,
        contact.first_name || "",
        contact.last_name || "",
        contact.created_at || "",
        contact.updated_at || "",
      ];

      if (includeAttributes) {
        row.push(contact.attribute ? JSON.stringify(contact.attribute) : "");
      }

      row.push(contact.lists.map((list) => list.list_name)?.join(","));

      // Escape fields that contain the delimiter
      return row
        .map((field) => {
          if (
            String(field).includes(delimiter) ||
            String(field).includes('"')
          ) {
            return `"${String(field).replace(/"/g, '""')}"`;
          }
          return String(field);
        })
        .join(delimiter);
    });

    // Join headers and rows
    return [headers.join(delimiter), ...rows].join("\n");
  };

  const handleExport = async () => {
    try {
      setIsExporting(true);
      const filteredContacts = await resolveContactsToExport();

      if (filteredContacts.length === 0) {
        toast({
          title: "No contacts to export",
          description: "No contacts match your filter criteria.",
          variant: "destructive",
        });
        setIsExporting(false);
        return;
      }

      const fileContent = formatContactsAsCSV(filteredContacts);
      const mimeType = "text/csv";
      const blob = new Blob([fileContent], { type: mimeType });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;

      // Generate file name based on list names
      const fileName =
        filteredContacts
          .flatMap((contact) => contact.lists.map((list) => list.list_name))
          .filter((name, index, self) => self.indexOf(name) === index) // Remove duplicates
          .join("_") || "contacts"; // Fallback name

      a.download = `${fileName}.csv`;

      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);

      toast({
        title: "Export successful",
        description: `Successfully exported ${filteredContacts.length} contacts.`,
        variant: "default",
      });
    } catch (error) {
      toast({
        title: "Export failed",
        description: "Something went wrong. Please try again.",
        variant: "destructive",
      });
      console.error("Export error:", error);
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className="container p-4 sm:p-6 max-w-6xl">
      <Card>
        <CardHeader>
          <CardTitle>Export subscribers</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
            {/* Include Attributes Switch */}
            <div className="flex flex-col justify-between">
              <Label>Include Attributes?</Label>
              <div className="flex items-center space-x-2 mt-2">
                <Switch
                  checked={includeAttributes}
                  onCheckedChange={setIncludeAttributes}
                />
                <span className="text-sm text-muted-foreground">
                  Include additional attributes
                </span>
              </div>
              <div className="text-xs text-muted-foreground mt-1">
                Includes all custom attributes in export
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
                placeholder="Select lists to export from..."
                variant="default"
                className="w-full"
                maxCount={listOptions.length}
              />
            )}
            <p className="text-xs text-muted-foreground">
              Only export subscribers from these lists (leave empty to export
              all)
            </p>
          </div>

          {/* Example */}
          <div className="space-y-2">
            <Label>Example CSV Export</Label>
            <div className="bg-gray-100 p-3 sm:p-4 rounded-lg overflow-x-auto">
              <pre className="text-xs sm:text-sm whitespace-pre-wrap">
                {`email${delimiter}first_name${delimiter}last_name${delimiter}created${delimiter}updated${
                  includeAttributes ? `${delimiter}attributes` : ""
                }${delimiter}lists
saisabk29@.com${delimiter}Saisab${delimiter}Karki${delimiter}2025-06-15T09:30:00Z${delimiter}2025-07-20T15:40:00Z${
                  includeAttributes
                    ? `${delimiter}"{\\"location\\":\\"New York\\",\\"interests\\":\\"sports,tech\\"}"`
                    : ""
                }${delimiter}"List A;List B"
sarah@example.com${delimiter}Sarah${delimiter}Jones${delimiter}2023-06-16T14:22:00Z${delimiter}2023-06-16T14:22:00Z${
                  includeAttributes
                    ? `${delimiter}"{\\"location\\":\\"London\\",\\"referral\\":\\"website\\"}"`
                    : ""
                }${delimiter}"List C"`}
              </pre>
            </div>
          </div>

          {/* Export Button */}
          <Button
            className="w-full"
            size="lg"
            onClick={handleExport}
            disabled={isExporting}
          >
            <FileDown className="mr-2 h-5 w-5" />
            {isExporting ? "Exporting..." : "Export Contacts"}
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
