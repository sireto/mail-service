import React from "react";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@/components/ui/dropdown-menu";
import { ChevronDown, Mail } from "lucide-react";

const Header = () => {
  return (
    <header className="flex justify-between items-center p-4 border-b bg-white">
      <div className="flex items-center space-x-2">
        <Mail color="#0070d6" />
        <span className="text-lg font-semibold text-black">Mail Service</span>
      </div>

      <DropdownMenu>
        <DropdownMenuTrigger className="flex items-center space-x-2 cursor-pointer outline-none">
          <Avatar>
            <AvatarFallback>A</AvatarFallback>
          </Avatar>
          <ChevronDown className="w-4 h-4 text-[#0070d6]" />
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="text-[#0070d6]">
          <DropdownMenuItem>Profile</DropdownMenuItem>
          <DropdownMenuItem>Settings</DropdownMenuItem>
          <DropdownMenuItem>Logout</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </header>
  );
};

export default Header;
