import React, { useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { formatDate } from "@/lib/utils";
import { Edit3, Trash2 } from "lucide-react";
import { EditCampaignSender } from "./_components/EditCampaignSender";
import { useDeleteCampaignSenderMutation } from "@/app/services/CampaignSenderApi";

interface CampaignSenderActionsProps {
  senderData: {
    id: string;
    from_name: string;
    from_email: string;
    updated_at: string;
    created_at: string;
  };
}

const CampaignSenderActions = ({ senderData }: CampaignSenderActionsProps) => {
  const [isOpen, setIsOpen] = useState(false);
  const [deleteSender] = useDeleteCampaignSenderMutation();

  const handleDelete = async () => {
    if (confirm("Are you sure you want to delete this sender?")) {
      try {
        await deleteSender(senderData.id).unwrap();
      } catch (error) {
        console.error("Error deleting sender:", error);
      }
    }
  };

  return (
    <div className="flex space-x-4 items-center">
      <button onClick={() => setIsOpen(true)}>
        <Edit3 className="w-5 h-5 text-blue-500" />
      </button>
      <button onClick={handleDelete}>
        <Trash2 size={20} strokeWidth={1.5} className="text-red-400" />
      </button>
      <EditCampaignSender
        open={isOpen}
        onClose={() => setIsOpen(false)}
        senderData={senderData}
      />
    </div>
  );
};

export const columns: ColumnDef<any>[] = [
  {
    accessorKey: "from_name",
    header: "From Name",
    cell: ({ row }) => <span>{row.getValue("from_name")}</span>,
  },
  {
    accessorKey: "from_email",
    header: "From Email",
    cell: ({ row }) => <span>{row.getValue("from_email")}</span>,
  },
  {
    accessorKey: "created_at",
    header: "Created",
    cell: ({ row }) => <span>{formatDate(row.getValue("created_at"))}</span>,
  },
  {
    accessorKey: "updated_at",
    header: "Updated",
    cell: ({ row }) => <span>{formatDate(row.getValue("updated_at"))}</span>,
  },
  {
    id: "actions",
    enableSorting: false,
    header: () => "Actions",
    cell: ({ row }) => <CampaignSenderActions senderData={row.original} />,
  },
];

export default columns;
