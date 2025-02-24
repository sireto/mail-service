'use client';

import { TableOfContents, ArchiveIcon, Rocket, ChartColumn } from 'lucide-react';
import Link from 'next/link';
import { useParams, usePathname } from 'next/navigation';

const FormNavLinks = [
  { title: 'Campaign', path: '/dashboard/campaigns/[id]', icon: <Rocket size={20}/> },
  { title: 'Content', path: '/dashboard/campaigns/[id]/content', icon: <TableOfContents size={20}/> },
  { title: 'Archive', path: '/dashboard/campaigns/[id]/archive', icon: <ArchiveIcon size={20}/> },
  { title: 'Analytics', path: '/dashboard/campaigns/[id]/analytics', icon: <ChartColumn size={20}/> },
]

export default function NewLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const { id } = useParams(); // Get campaign ID from URL parameters
  const pathname = usePathname();

  return (
    <div className="w-full max-w-6xl lg:p-6 no-scrollbar">
      <h2 className="text-2xl font-semibold text-gray-900 mb-6">Campaigns</h2>
      
      {/* Sub-navigation */}
      <nav className="flex space-x-4 border-b pb-3 mb-6 overflow-x-scroll lg:overflow-x-hidden no-scrollbar">
        {FormNavLinks.map((link) => {
          // Replace the [id] placeholder with the actual campaign ID
          const linkPath = link.path.replace('[id]', Array.isArray(id) ? id[0] : id ?? 'new');

          const isActive = linkPath === pathname; // Check if the current path matches the link

          return (
            <Link
              key={link.path}
              href={linkPath}
              className={`flex items-center gap-2 px-4 py-2 rounded-lg text-gray-600 hover:text-gray-900 transition-all duration-300 ${isActive ? 'bg-gray-100 !text-primary' : ''}`}
            >
              {link.icon}
              <span className="text-sm font-medium">{link.title}</span>
            </Link>
          );
        })}
      </nav>

      <div className="flex-1 overflow-auto p-4 bg-white lg:border-l border-gray-200">
        {children}
      </div>
    </div>
  );
}