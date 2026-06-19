// "use client";
// import React from "react";

// import { sepolia, mainnet } from "@stellar-react/chains";
// import {
//   StellarConfig,
//   publicProvider,
//   argent,
//   braavos,
//   useInjectedConnectors,
//   voyager,
 
// } from "@stellar-react/core";

// export function StellarProvider({ children }: { children: React.ReactNode }) {
 
//   const { connectors } = useInjectedConnectors({

//     recommended: [argent(), braavos()],
//     //includeRecommended: "onlyIfNoConnectors",
//     order: "alphabetical",
//   });

  
//   return (
//     <StellarConfig
//       chains={[mainnet, sepolia]}
//       provider={publicProvider()}
//       connectors={connectors}
//       explorer={voyager}
//     >
//       {children}
//     </StellarConfig>
//   );
// }
