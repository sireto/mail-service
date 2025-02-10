// app/dashboard/layout.tsx
import Sidebar from "@/components/Sidebar";

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="flex h-[calc(100vh-64px)]">
      {" "}
      {/* Adjust 64px based on your header height */}
      <Sidebar />
      <div className="flex-1 overflow-auto p-8">{children}</div>
    </div>
  );
}
