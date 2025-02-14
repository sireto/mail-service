"use client";

import React from "react";
import Link from "next/link";
import { cn } from "@/lib/utils";
import { usePathname } from "next/navigation";
import {
  Home,
  List,
  Users,
  Settings,
  Rocket,
  User,
  LayoutPanelTop,
  Server,
  X,
} from "lucide-react";

const sidebarItems = [
  { name: "Dashboard", href: "/dashboard", icon: Home },
  { name: "Lists", href: "/dashboard/lists", icon: List },
  { name: "Contacts", href: "/dashboard/contacts", icon: Users },
  { name: "Templates", href: "/dashboard/templates", icon: LayoutPanelTop },
  { name: "Campaigns", href: "/dashboard/campaigns", icon: Rocket },
  { name: "Servers", href: "/dashboard/servers", icon: Server },
  { name: "Users", href: "/dashboard/users", icon: User },
  { name: "Settings", href: "/dashboard/settings", icon: Settings },
];

interface SidebarProps {
  onClose?: () => void;
}

const Sidebar = ({ onClose }: SidebarProps) => {
  const pathname = usePathname();
  const normalizedPathname = pathname ? pathname.replace(/\/$/, "") : "";

  return (
    <aside className="w-64 h-screen border-r bg-white relative">
      {/* Mobile Close Button */}
      {onClose && (
        <button
          onClick={onClose}
          className="absolute top-4 right-4 md:hidden"
          aria-label="Close sidebar"
        >
          <X className="w-6 h-6" />
        </button>
      )}

      <nav className="flex flex-col space-y-1 p-4 pt-16 md:pt-4">
        {sidebarItems.map((item, index) => (
          <Link
            key={index}
            href={item.href}
            className={cn(
              "flex items-center space-x-3 px-4 py-2 rounded-md text-gray-700 hover:bg-gray-100 hover:text-gray-900 transition",
              normalizedPathname === item.href
                ? "text-blue-600 font-medium border-r-4 border-blue-600"
                : ""
            )}
            onClick={onClose}
          >
            <item.icon className="w-5 h-5" />
            <span>{item.name}</span>
          </Link>
        ))}
      </nav>
    </aside>
  );
};

export default Sidebar;
