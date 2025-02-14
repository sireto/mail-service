
// app/dashboard/layout.tsx
import Sidebar from "@/components/Sidebar";
import ReduxProvider from "@/providers/redux-provider";


export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="flex h-[calc(100vh-64px)]">
      {" "}
      <ReduxProvider>
        <Sidebar />
        <div className="flex-1 overflow-auto p-8">{children}</div>
      </ReduxProvider>
    </div>
  );
}
