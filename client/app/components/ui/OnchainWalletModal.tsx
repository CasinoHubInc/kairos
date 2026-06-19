"use client";
import React from "react";
import { useConnect } from "@stellar-react/core";
// import { Button } from "@/components/ui/button";
// import { useAppContext } from "@/app/context/appContext";
// import { useAccount, useDisconnect } from "@stellar-react/core";
import { StellarkitConnector, useStellarkitConnectModal } from "stellarkit";
import { toast } from "react-toastify";

const WalletModal = () => {
  const { connectAsync, connectors } = useConnect();

  const { stellarkitConnectModal } = useStellarkitConnectModal({
    connectors: connectors as StellarkitConnector[],
    modalTheme: "system",
  });
  return (
    <div className="p-6 max-w-md mx-auto  rounded-xl shadow-md space-y-4">
      <h2 className="text-xl font-semibold text-center">Connect Your Wallet</h2>
      <div className="space-y-2">

      </div>
      <button
        className="w-full justify-center"
        onClick={async () => {
          const { connector } = await stellarkitConnectModal();
          if (!connector) {
            console.log("User rejected to connect");
            return;
          }
          await connectAsync({ connector })
            .then(() => {
        
              toast.success("Wallet conectada");
              // Guardar el ID del conector en localStorage
              localStorage.setItem("connector", connector.id);
            })
            .catch((e) => {
              console.log(e);
              toast.error("Error al conectar la wallet");
            });
        }}
      >
        connect wallet
      </button>
    </div>
  );
};

export default WalletModal;
