import { Button } from "@/components/ui/button";
import { ContactForm } from "@/app/dashboard/contacts/_components/ContactForm";
import { useState } from "react";

interface List {
  id: string;
  name: string;
  description: string;
  namespace_id: string;
  created_at: string;
  updated_at: string;
}

interface NoContactsFoundProps {
  lists: List[] | null;
}

const NoContactsFound: React.FC<NoContactsFoundProps> = ({ lists }) => {
  const [openAddContactModal, setOpenAddContactModal] = useState(false);

  const handleAddContactClick = () => {
    setOpenAddContactModal(true);
  };

  return (
    <div className="flex flex-col items-center space-y-4">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        className="w-16 h-16 text-gray-500"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        strokeWidth="2"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          d="M9 12h6m2 4l-2-2 2-2m-6 4v2m0-10v2"
        />
      </svg>
      <p>No contacts found</p>
      <p>Please add a new contact.</p>
      <Button onClick={handleAddContactClick}>Add New Contact</Button>

      {/* Modal for adding a contact */}
      {openAddContactModal && (
        <ContactForm
          open={openAddContactModal}
          onClose={() => setOpenAddContactModal(false)}
          lists={lists}
        />
      )}
    </div>
  );
};

export default NoContactsFound;
