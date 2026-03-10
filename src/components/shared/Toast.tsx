import { useCallback, useState, type ReactNode } from "react";
import {
  X,
  CheckCircle2,
  AlertCircle,
  AlertTriangle,
  Info,
} from "lucide-react";
import {
  ToastContext,
  type Toast,
  type ToastInput,
  type ToastVariant,
} from "./toast-context";

const VARIANT_STYLES: Record<
  ToastVariant,
  { bg: string; border: string; icon: typeof Info; iconColor: string }
> = {
  success: {
    bg: "bg-green-500/10",
    border: "border-green-500/30",
    icon: CheckCircle2,
    iconColor: "text-green-400",
  },
  error: {
    bg: "bg-red-500/10",
    border: "border-red-500/30",
    icon: AlertCircle,
    iconColor: "text-red-400",
  },
  warning: {
    bg: "bg-amber-500/10",
    border: "border-amber-500/30",
    icon: AlertTriangle,
    iconColor: "text-amber-400",
  },
  info: {
    bg: "bg-blue-500/10",
    border: "border-blue-500/30",
    icon: Info,
    iconColor: "text-blue-400",
  },
};

const MAX_TOASTS = 5;
const AUTO_DISMISS_MS = 4000;

let toastCounter = 0;

function ToastItem({
  toast,
  onDismiss,
}: {
  toast: Toast;
  onDismiss: (id: number) => void;
}) {
  const style = VARIANT_STYLES[toast.variant];
  const Icon = style.icon;

  return (
    <div
      className={`flex items-start gap-2.5 rounded-lg border ${style.border} ${style.bg} px-3 py-2.5 shadow-lg backdrop-blur-sm animate-in slide-in-from-right`}
      role="alert"
    >
      <Icon
        size={16}
        className={`mt-0.5 shrink-0 ${style.iconColor}`}
        aria-hidden="true"
      />
      <div className="flex-1 min-w-0">
        <p className="text-sm font-medium text-text-primary">{toast.title}</p>
        {toast.description && (
          <p className="mt-0.5 text-xs text-text-secondary">
            {toast.description}
          </p>
        )}
      </div>
      <button
        onClick={() => onDismiss(toast.id)}
        className="shrink-0 rounded p-0.5 text-text-secondary hover:text-text-primary"
        aria-label="Dismiss"
      >
        <X size={14} aria-hidden="true" />
      </button>
    </div>
  );
}

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const dismiss = useCallback((id: number) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, []);

  const toast = useCallback(
    (input: ToastInput) => {
      const id = ++toastCounter;
      const newToast: Toast = {
        id,
        title: input.title,
        description: input.description,
        variant: input.variant ?? "info",
      };

      setToasts((prev) => {
        const next = [...prev, newToast];
        // Keep only the most recent toasts
        return next.length > MAX_TOASTS ? next.slice(-MAX_TOASTS) : next;
      });

      setTimeout(() => {
        dismiss(id);
      }, AUTO_DISMISS_MS);
    },
    [dismiss],
  );

  return (
    <ToastContext.Provider value={{ toast }}>
      {children}
      {/* Toast stack */}
      <div className="fixed bottom-4 right-4 z-50 flex flex-col gap-2 w-80">
        {toasts.map((t) => (
          <ToastItem key={t.id} toast={t} onDismiss={dismiss} />
        ))}
      </div>
    </ToastContext.Provider>
  );
}
