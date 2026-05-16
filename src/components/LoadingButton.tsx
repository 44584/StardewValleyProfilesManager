import type { ReactNode } from 'react';

interface LoadingButtonProps {
  children: ReactNode;
  onClick?: () => void;
  loading?: boolean;
  disabled?: boolean;
  variant?: 'primary' | 'secondary' | 'danger' | 'success';
  type?: 'button' | 'submit';
  className?: string;
}

export default function LoadingButton({
  children,
  onClick,
  loading = false,
  disabled = false,
  variant = 'primary',
  type = 'button',
  className = '',
}: LoadingButtonProps) {
  return (
    <button
      type={type}
      className={`btn btn-${variant} ${loading ? 'btn-loading' : ''} ${className}`}
      onClick={onClick}
      disabled={disabled || loading}
    >
      {loading && <span className="spinner" />}
      {children}
    </button>
  );
}
