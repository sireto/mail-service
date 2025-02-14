'use client';

import { TableOfContents, ArchiveIcon, Rocket, ChartColumn } from 'lucide-react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';

const FormNavLinks = [
    { title: 'Campaign', path: '/dashboard/campaigns/new', icon: <Rocket size={20}/> },
    { title: 'Content', path: '/dashboard/campaigns/new/content', icon: <TableOfContents size={20}/> },
    { title: 'Archive', path: '/dashboard/campaigns/new/archive', icon: <ArchiveIcon size={20}/> },
    { title: 'Analytics', path: '/dashboard/campaigns/new/analytics', icon: <ChartColumn size={20}/> },
]

export default function NewLayout({
  children,
}: {
  children: React.ReactNode;
}) {
    const pathname = usePathname();
    
  return (
    <div className="w-full max-w-6xl p-6">
      <h2 className="text-2xl font-semibold text-gray-900 mb-6">Campaigns</h2>
      {/* New Campaign */}
      {/* Sub-navigation */}
      <nav className="flex space-x-4 border-b pb-3 mb-6">
        {FormNavLinks.map((link) => {
            const isActive = pathname === link.path;

            return (
              <Link
                key={link.path}
                href={link.path}
                className={
                  `flex items-center gap-2 px-4 py-2 rounded-lg text-gray-600 hover:text-gray-900 transition-all duration-300 ${isActive ? 'bg-gray-100 !text-primary' : ''}`
                }
              >
                {link.icon}
                <span className="text-sm font-medium">{link.title}</span>
              </Link>
            )
        })
        }
      </nav>

      <div className="flex-1 overflow-auto p-4 bg-white border-l border-gray-200">
        {children}
      </div>
    </div>
  );
}
