import { useEffect, useState } from 'react';
import type { PageComponent } from '@pages/route-map';

const UrlReader: PageComponent = () => {
  const [responseMessage, setResponseMessage] = useState('');
  const [scannedResult, setScannedResult] = useState<string | undefined>('');
  const [inputURL, setInputURL] = useState<string>('');

  const sendURL = async () => {
    try {
      console.log('sending ...');
      console.log(scannedResult);
      const response = await fetch('http://localhost:3000/api/v1/qr-analyse', {
        method: 'POST',
        body: JSON.stringify(scannedResult),
        headers: {
          'Content-Type': 'application/json',
        },
      });

      const result = await response.json();
      console.log(result);
      setResponseMessage(result);
    } catch (error) {
      console.error('Error:', error);
    }
  };

  // useEffect outside handleSubmit
  useEffect(() => {
    if (scannedResult) {
      sendURL();
    }
  }, [scannedResult, sendURL]); // Trigger useEffect when scannedResult changes

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    setScannedResult(inputURL); // This will trigger the useEffect
  };

  return (
    <div className="relative m-0 h-screen w-full md:w-3/4">
      <div className="left-0 w-full">
        <form onSubmit={handleSubmit} className="absolute left-5 top-5 z-50">
          <input
            type="text"
            value={inputURL}
            onChange={(e) => setInputURL(e.target.value)}
            placeholder="Enter URL"
            className="border p-2"
          />
          <button type="submit" className="ml-2 bg-blue-500 p-2 text-white">
            Submit
          </button>
        </form>
      </div>

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
            <>
              <p style={{ marginTop: '10px', color: 'green' }}>
                Server response: {responseMessage}
              </p>
            </>
          )}
        </p>
      )}
    </div>
  );
};

export default UrlReader;
