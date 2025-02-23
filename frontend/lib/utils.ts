import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 *
 * @param dateString
 * @description Format a date string to a en-US locale date string...
 * @returns
 */
export function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString("en-US", {
    weekday: "short",
    day: "2-digit",
    month: "short",
    year: "numeric",
  });
}

/**
 *
 * @param dateString
 * @description Format a date string to a en-US locale date string without day...
 * @returns
 */
export function formatDateWithoutDay(dateString: string) {
  const date = new Date(dateString);
  const localDate = date.toLocaleDateString("ne-NP", {
    day: "2-digit",
    month: "short",
    year: "numeric",
  });
  const localTime = date.toLocaleTimeString("ne-NP", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: true, // Use 12-hour format (AM/PM)
    timeZone: "Asia/Kathmandu",
  });

  return { localDate, localTime };
}

/**
 * 
 * @param date 
 * @description Set the time of a date to the end of the day by time...
 * @returns 
 */
export function setToEndOfDay(date: Date) {
  const d = new Date(date);
  d.setHours(23, 59, 59, 999);
  return d;
}