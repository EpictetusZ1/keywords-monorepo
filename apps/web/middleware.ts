import { NextRequest, NextResponse } from 'next/server'

export function middleware(request: NextRequest) {
    const ContentSecurityPolicy = `
  default-src 'self';
  script-src 'self' 'unsafe-eval' 'unsafe-inline' https://va.vercel-scripts.com https://vitals.vercel-insights.com;
  child-src 'self' blob: https://js.stripe.com;
  worker-src 'self' blob:;
  style-src 'self' 'unsafe-inline';
  font-src 'self';
  frame-src 'self';
  img-src 'self';
  object-src data:;
  connect-src 'self' vitals.vercel-insights.com http://localhost:8080 http://127.0.0.1:8080 http://127.0.0.1:8080/submit;
`.trim()

    // Replace newline characters and spaces
    const contentSecurityPolicyHeaderValue = ContentSecurityPolicy
        .replace(/\s{2,}/g, ' ')
        .trim()

    const requestHeaders = new Headers(request.headers)

    requestHeaders.set(
        'Content-Security-Policy',
        contentSecurityPolicyHeaderValue
    )

    const response = NextResponse.next({
        request: {
            headers: requestHeaders,
        },
    })
    response.headers.set(
        'Content-Security-Policy',
        contentSecurityPolicyHeaderValue
    )

    return response
}