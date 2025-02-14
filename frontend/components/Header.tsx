"use client";
import React, { useState } from "react";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@/components/ui/dropdown-menu";
import { ChevronDown, Mail, Menu, X } from "lucide-react";
import Sidebar from "./Sidebar";
import { cn } from "@/lib/utils";

const Header = () => {
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);

  const toggleSidebar = () => {
    setIsSidebarOpen(!isSidebarOpen);
  };

  return (
    <>
      <header className="flex justify-between items-center p-4 border-b bg-white">
        <div className="flex items-center space-x-2">
          {/* Mobile Hamburger Menu */}
          <button
            className="md:hidden mr-2"
            onClick={toggleSidebar}
            aria-label={isSidebarOpen ? "Close menu" : "Open menu"}
          >
            {isSidebarOpen ? (
              <X className="w-6 h-6" />
            ) : (
              <Menu className="w-6 h-6" />
            )}
          </button>

          <Mail color="#0070d6" />
          <span className="text-lg font-semibold text-black">Mail Service</span>
        </div>

        {/* Desktop Dropdown Menu */}
        <div className="hidden md:block">
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
        </div>

        {/* Mobile Menu */}
        <div className="md:hidden">
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
        </div>
      </header>

      {/* Mobile Sidebar Overlay */}
      <div
        className={cn(
          "fixed inset-0 z-40 bg-black/50 md:hidden",
          isSidebarOpen ? "block" : "hidden"
        )}
        onClick={toggleSidebar}
      />

      {/* Mobile Sidebar */}
      <div
        className={cn(
          "fixed top-0 left-0 h-full w-64 z-50 transform transition-transform duration-300 ease-in-out md:hidden",
          isSidebarOpen ? "translate-x-0" : "-translate-x-full"
        )}
      >
        <Sidebar onClose={toggleSidebar} />
      </div>
    </>
  );
};

export default Header;
