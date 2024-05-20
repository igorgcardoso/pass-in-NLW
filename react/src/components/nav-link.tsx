import { ComponentProps } from "react";

interface NavLinkProps extends ComponentProps<"a"> {}

export function NavLink(props: NavLinkProps) {
  return (
    <a {...props} className="font-medium text-sm">
      {props.children}
    </a>
  );
}
