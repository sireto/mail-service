import { ChartSpline, Table2 } from "lucide-react";
import React from "react";

const ViewModeToggler = ({
  isGraphView,
  setIsGraphView,
}: {
  isGraphView: boolean;
  setIsGraphView: (state: boolean) => void;
}) => {
  return (
    <div className="flex justify-center items-center bg-secondary rounded overflow-hidden">
      <button
        className={`flex items-center justify-center h-full px-3 transition-all duration-300 ease-in-out ${!isGraphView ? "text-primary-foreground bg-primary" : "text-primary hover:opacity-60"}`}
        onClick={() => setIsGraphView(false)}
      >
        <Table2 size={16} />
      </button>
      <button
        className={`flex items-center justify-center h-full px-3 transition-all duration-300 ease-in-out ${isGraphView ? "text-primary-foreground bg-primary" : "text-primary hover:opacity-60"}`}
        onClick={() => setIsGraphView(true)}
      >
        <ChartSpline size={16} />
      </button>
    </div>
  );
};

export default ViewModeToggler;
