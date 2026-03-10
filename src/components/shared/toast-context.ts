import { createContext } from "react";

export type ToastVariant = "success" | "error" | "warning" | "info";

export interface Toast {
  id: number;
  title: string;
  description?: string;
  variant: ToastVariant;
}

export interface ToastInput {
  title: string;
  description?: string;
  variant?: ToastVariant;
}

interface ToastContextValue {
  toast: (input: ToastInput) => void;
}

export const ToastContext = createContext<ToastContextValue | null>(null);
