// components/NoServersFound.tsx
import { Button } from "@/components/ui/button";
import ServerCard from "@/components/ServerCard";
import { useState } from "react";

const NoServersFound = () => {
  const [showNewServerCard, setShowNewServerCard] = useState(false);

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
      <p>No servers found</p>
      <p>Please add your server settings.</p>
      <Button onClick={() => setShowNewServerCard(true)}>Add New Server</Button>
      {showNewServerCard && <ServerCard key="new" server={{}} />}
    </div>
  );
};

export default NoServersFound;
