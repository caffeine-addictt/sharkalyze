/**
 * SPDX-FileCopyrightText: 2024 Ng Jun Xiang <contact@ngjx.org>
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

import type { RouteMap } from '@pages/route-map';

import QrReader from './qr';

const qrRouteMap: RouteMap = {
  '/qr-reader': {
    title: 'QR',
    description: 'Lets Validate',
    component: QrReader,
  },
} as const;
export default qrRouteMap;
