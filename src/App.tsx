import { useState, useRef, useEffect, useCallback } from "react";
import { createCdm } from "@dotdm/cdm";

export function App() {
  const [count, setCount] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [incrementing, setIncrementing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const cdmRef = useRef<ReturnType<typeof createCdm> | null>(null);
  const counterRef = useRef<any>(null);

  useEffect(() => {
    const cdm = createCdm();
    cdmRef.current = cdm;
    counterRef.current = cdm.getContract("@example/counter");

    return () => {
      cdm.destroy();
    };
  }, []);

  const queryCount = useCallback(async () => {
    if (!counterRef.current) return;
    setLoading(true);
    setError(null);
    try {
      const result = await counterRef.current.getCount.query();
      setCount(result.value);
    } catch (e: any) {
      setError(e.message ?? "Query error");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (counterRef.current) queryCount();
  }, [queryCount]);

  const handleIncrement = async () => {
    if (!counterRef.current) return;
    setIncrementing(true);
    setError(null);
    try {
      await counterRef.current.increment.tx();
      await queryCount();
    } catch (e: any) {
      setError(e.message ?? "Transaction error");
    } finally {
      setIncrementing(false);
    }
  };

  return (
    <div style={{ fontFamily: "system-ui, sans-serif", maxWidth: 420, margin: "80px auto", textAlign: "center" }}>
      <h1>Counter</h1>

      <div style={{ fontSize: 64, fontWeight: "bold", margin: "32px 0" }}>
        {loading ? "..." : count ?? "?"}
      </div>

      {error && (
        <p style={{ color: "red", fontSize: 14 }}>{error}</p>
      )}

      <div style={{ display: "flex", gap: 12, justifyContent: "center" }}>
        <button
          onClick={handleIncrement}
          disabled={incrementing}
          style={{
            padding: "12px 24px",
            fontSize: 16,
            cursor: incrementing ? "wait" : "pointer",
            borderRadius: 8,
            border: "1px solid #ccc",
            background: incrementing ? "#eee" : "#fff",
          }}
        >
          {incrementing ? "Incrementing..." : "Increment"}
        </button>
        <button
          onClick={queryCount}
          disabled={loading}
          style={{
            padding: "12px 24px",
            fontSize: 16,
            cursor: loading ? "wait" : "pointer",
            borderRadius: 8,
            border: "1px solid #ccc",
            background: loading ? "#eee" : "#fff",
          }}
        >
          {loading ? "Loading..." : "Refresh"}
        </button>
      </div>
    </div>
  );
}
