/**
 * SPDX-FileCopyrightText: 2024 Ng Jun Xiang <contact@ngjx.org>
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

import type { PageComponent } from '@pages/route-map';

const RootPage: PageComponent = ({ className, ...props }) => {
  return (
    <div className={className} {...props}>
      root page
    </div>
  );
};
export default RootPage;
