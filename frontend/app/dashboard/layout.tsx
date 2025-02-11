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
      {/**minus header height*/}
      <Sidebar />
      <main className="flex-1 overflow-y-auto relative">
        <div className="p-8">{children}</div>
      </main>
    </div>
  );
}
