import { useEffect, useRef, useState } from 'react';
import type { PageComponent } from '@pages/route-map';

// Qr Scanner
import QrScanner from 'qr-scanner';
import QrFrame from '../../assets/qr-frame.svg';

const QrReader: PageComponent = () => {
  // QR States
  const scanner = useRef<QrScanner>();
  const videoEl = useRef<HTMLVideoElement>(null);
  const qrBoxEl = useRef<HTMLDivElement>(null);
  const [qrOn, setQrOn] = useState<boolean>(true);
  const [responseMessage, setResponseMessage] = useState('');

  // Result
  const [scannedResult, setScannedResult] = useState<string | undefined>('');

  // Success
  const onScanSuccess = (result: QrScanner.ScanResult) => {
    // 🖨 Print the "result" to browser console.
    console.log(result);
    setScannedResult(result?.data);
    // Effect to trigger sendURL when scannedResult changes
  };

  useEffect(() => {
    const sendURL = async () => {
      try {
        console.log('sending ...');
        console.log(scannedResult);
        const response = await fetch(
          'http://localhost:3000/api/v1/qr-analyse',
          {
            method: 'POST',
            body: JSON.stringify(scannedResult),
            headers: {
              'Content-Type': 'application/json',
            },
          },
        );

        const result = await response.json();
        console.log(result);
        setResponseMessage(result.message);
      } catch (error) {
        console.error('Error:', error);
      }
    };

    if (scannedResult) {
      sendURL();
    }
  }, [scannedResult]);

  // Fail
  const onScanFail = (err: string | Error) => {
    // 🖨 Print the "err" to browser console.
    console.log(err);
  };

  useEffect(() => {
    if (videoEl?.current && !scanner.current) {
      // Instantiate the QR Scanner
      scanner.current = new QrScanner(videoEl?.current, onScanSuccess, {
        onDecodeError: onScanFail,
        preferredCamera: 'environment',
        highlightScanRegion: true,
        highlightCodeOutline: true,
        overlay: qrBoxEl?.current || undefined,
      });

      // Start QR Scanner
      scanner?.current
        ?.start()
        .then(() => setQrOn(true))
        .catch((err: string) => {
          if (err) setQrOn(false);
        });
    }

    // Store the current value of videoEl.current in a variable
    const videoElement = videoEl.current;

    // Clean up on unmount.
    return () => {
      // Use the stored variable instead of videoEl.current
      if (videoElement) {
        scanner?.current?.stop();
      }
    };
  });

  // If "camera" is not allowed in browser permissions, show an alert.
  useEffect(() => {
    if (!qrOn)
      alert(
        'Camera is blocked or not accessible. Please allow camera in your browser permissions and Reload.',
      );
  }, [qrOn]);

  return (
    <div className="relative m-0 h-screen w-full md:w-3/4">
      {/* QR */}
      <video ref={videoEl} className="size-full object-cover"></video>
      <div ref={qrBoxEl} className="left-0 w-full">
        <img
          src={QrFrame}
          alt="Qr Frame"
          width={256}
          height={256}
          className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 fill-none"
        />
      </div>

      {/* Show Data Result if scan is success */}
      {scannedResult && (
        <p
          style={{
            position: 'absolute',
            top: 0,
            left: 0,
            zIndex: 99999,
            color: 'white',
          }}
        >
          Scanned Result: {scannedResult}
          {responseMessage && (
            <p style={{ marginTop: '10px', color: 'green' }}>
              {' '}
              server response: {responseMessage}
            </p>
          )}
        </p>
      )}
    </div>
  );
};

export default QrReader;
