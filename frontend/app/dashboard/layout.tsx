"use client";

import Sidebar from "@/components/Sidebar";
import ReduxProvider from "@/providers/redux-provider";

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <ReduxProvider>
      <div className="flex h-[calc(100vh-64px)]">
        <div className="hidden md:block w-64">
          <Sidebar />
        </div>

        {/* Main content */}
        <div className="flex-1 flex flex-col">
          <main className="flex-1 overflow-auto p-8">{children}</main>
        </div>
      </div>
    </ReduxProvider>
  );
}
