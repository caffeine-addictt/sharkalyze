/**
 * SPDX-FileCopyrightText: 2024 Ng Jun Xiang <contact@ngjx.org>
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

import * as React from 'react';

import ReactDOM from 'react-dom/client';
import { useLocation, Route, Routes, BrowserRouter } from 'react-router-dom';
import '@styles/globals.css';

import { Helmet } from 'react-helmet';
import routes, { type RouteDetails } from '@pages/route-map';

// Components
import Navbar from '@components/navbar';
import Footer from '@components/footer';

export const WrappedComponent = ({
  component: Component,
  path,
  title,
  description,
}: RouteDetails & { path: string }): JSX.Element | null => {
  return (
    <>
      <Helmet titleTemplate={path !== '/' ? '%s | GreenBitesSG' : '%s'}>
        <title>{title}</title>
        <meta name="description" content={description ?? 'Sharkalyze'} />
      </Helmet>
      <Component className="flex w-full max-w-full grow" />
    </>
  );
};

export const Layout = (): JSX.Element => {
  const location = useLocation();

  return (
    <div className="flex min-w-full max-w-full flex-col bg-background-light text-text-light dark:bg-background-dark  dark:text-text-dark">
      <main className="flex min-h-screen flex-col">
        <Navbar />

        <Routes location={location}>
          {Object.entries(routes).map(([path, details], i) => (
            <Route
              key={i}
              path={path}
              element={
                <WrappedComponent {...details} path={location.pathname} />
              }
            />
          ))}
        </Routes>
      </main>

      <Footer />
    </div>
  );
};

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <Layout />
    </BrowserRouter>
  </React.StrictMode>,
);
