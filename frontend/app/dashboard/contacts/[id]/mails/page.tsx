"use client";

import {
  useGetContactByIdQuery,
  useGetMailsForContactQuery,
} from "@/app/services/ContactApi";
import { useDeleteMailMutation } from "@/app/services/MailApi";
import DataTable from "@/components/DataTable";
import { useParams } from "next/navigation";
import columns from "@/app/dashboard/contacts/[id]/mails/_columns";
import { User } from "lucide-react";
import Link from "next/link";

const Page = () => {
  const { id }: { id: string } = useParams();
  const { data: mails, isLoading } = useGetMailsForContactQuery(id);
  const { data: contact } = useGetContactByIdQuery(id);
  const [deleteMail, { error: deletionError }] = useDeleteMailMutation();

  const deleteMailHandler = async (id: string) => {
    if (deletionError) {
      return <div>Error deleting the mail</div>;
    }

    await deleteMail(id);
  };

  return (
    <>
      <div className="mb-6 flex items-center gap-x-4">
        <div className="text-2xl font-semibold text-gray-900 ">
          <User
            size={48}
            strokeWidth={1.5}
            className="min-w-[24px] min-h-[24px]"
          />
        </div>
        <div>
          <h2 className="text-xl font-semibold text-gray-900">
            {contact?.first_name} {contact?.last_name}
          </h2>
          <span className="px-2 py-1 text-xs rounded-full bg-gray-200 text-gray-700">
            {contact?.email}
          </span>
        </div>
      </div>
      {/* breadcrumb */}
      <div className="flex items-center gap-x-2 my-4 text-gray-500">
        <Link href={"/dashboard/"} className="hover:underline">
          Dashboard
        </Link>
        <span>/</span>
        <Link href={"/dashboard/contacts"} className="hover:underline">
          Contacts
        </Link>
        <span>/</span>
        <span>
          {contact?.first_name} {contact?.last_name}
        </span>
        <span>/</span>
        <span className="text-primary">mails</span>
      </div>
      <div className="my-8 z-20">
        <DataTable
          data={mails || []}
          columns={columns(deleteMailHandler)}
          fallback={"No mails found"}
          isLoading={isLoading}
        />
      </div>
    </>
  );
};

export default Page;
