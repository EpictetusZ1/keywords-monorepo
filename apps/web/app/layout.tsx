import "./globals.css";
import { Inter } from "next/font/google";
import { ReactElement, ReactNode } from "react";

const inter = Inter({ subsets: ["latin"] })

export default function RootLayout({children}: { children: ReactNode }): ReactElement {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  )
}
