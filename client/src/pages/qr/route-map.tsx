/**
 * SPDX-FileCopyrightText: 2024 Ng Jun Xiang <contact@ngjx.org>
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

import type { RouteMap } from '@pages/route-map';

import QrReader from './qr';
import UrlReader from './url';

const qrRouteMap: RouteMap = {
  '/qr-reader': {
    title: 'QR',
    description: 'Lets Validate',
    component: QrReader,
  },
  '/url-reader': {
    title: 'url',
    description: 'url',
    component: UrlReader,
  },
} as const;
export default qrRouteMap;
