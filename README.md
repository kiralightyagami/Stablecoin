# Stablecoin Protocol

A decentralized stablecoin protocol built on Solana using the Anchor framework. This protocol allows users to deposit SOL as collateral and mint stablecoins while maintaining proper health factors and liquidation mechanisms.

## Architecture

This stablecoin protocol implements a collateralized debt position (CDP) system where:
- Users deposit SOL as collateral
- Stablecoins are minted against the collateral
- Health factors ensure over-collateralization
- Liquidation mechanisms protect the protocol

## Features

- **Collateral Management**: Deposit and withdraw SOL collateral
- **Token Minting**: Mint stablecoins against deposited collateral
- **Health Factor Monitoring**: Automated health factor calculations using Pyth price feeds
- **Liquidation System**: Liquidate undercollateralized positions
- **Admin Controls**: Configuration management for protocol parameters

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18+)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.31.1)
- [Node.js](https://nodejs.org/) (v16+)
- [Yarn](https://yarnpkg.com/)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/kiralightyagami/Stablecoin
   cd Stablecoin
   ```

2. **Install dependencies**
   ```bash
   cd Contract
   yarn install
   ```

3. **Build the program**
   ```bash
   anchor build
   ```

4. **Configure Solana CLI**
   ```bash
   solana config set --url localhost
   solana-keygen new --outfile ~/.config/solana/id.json
   ```

5. **Start local validator**
   ```bash
   solana-test-validator
   ```

6. **Deploy the program**
   ```bash
   anchor deploy
   ```


## Program Instructions

### Core Instructions

1. **`init_config`**
   - Initializes the protocol configuration
   - Sets up the mint account and initial parameters

2. **`update_config`**
   - Updates protocol parameters (admin only)
   - Modifies minimum health factor requirements

3. **`deposit_collateral_and_mint_tokens`**
   - Deposits SOL as collateral
   - Mints stablecoins based on collateral value and health factor

4. **`redeem_collateral_and_burn_tokens`**
   - Burns stablecoins to redeem collateral
   - Ensures health factor remains above minimum

5. **`liquidate`**
   - Liquidates undercollateralized positions
   - Provides liquidation bonuses to liquidators

## Program Accounts

### Config Account
- **Authority**: Protocol admin public key
- **Mint Account**: Stablecoin token mint
- **Min Health Factor**: Minimum required health factor
- **Liquidation Threshold**: Threshold for liquidation eligibility
- **Liquidation Bonus**: Bonus percentage for liquidators

### Collateral Account
- **SOL Account**: PDA holding deposited SOL
- **Token Account**: User's stablecoin token account
- **Depositor Account**: User's public key
- **Lamport Balance**: Amount of SOL deposited
- **Amount Minted**: Amount of stablecoins minted

## Configuration

The protocol uses several key parameters:

- **Minimum Health Factor**: Ensures over-collateralization
- **Liquidation Threshold**: Determines when positions can be liquidated
- **Liquidation Bonus**: Incentivizes liquidators

These parameters can be updated by the protocol authority using the `update_config` instruction.

## Price Feeds

The protocol integrates with [Pyth Network](https://pyth.network/) for real-time SOL/USD price feeds:
- **Price Feed ID**: `0xfe650f0367d4a7ef9815a593ea15d36593f0643aaaf0149bb04be67ab851decd`
- **Network**: Solana Mainnet-Beta

## Project Structure

```
Contract/
├── programs/Stablecoin/src/
│   ├── lib.rs              # Main program entry point
│   ├── state.rs            # Account structures
│   ├── constants.rs        # Protocol constants
│   ├── error.rs            # Custom error definitions
│   └── instructions/       # Instruction implementations
│       ├── admin/          # Admin-only instructions
│       ├── deposit/        # Deposit and mint logic
│       ├── withdraw/       # Withdraw and burn logic
│       └── utils.rs        # Utility functions
├── tests/                  # Integration tests
├── target/                 # Build artifacts
└── migrations/             # Deployment scripts
```

## Security Considerations

- **Health Factor Monitoring**: Positions must maintain minimum health factors
- **Price Feed Validation**: Pyth price feeds are validated for freshness and accuracy
- **Access Controls**: Admin functions are restricted to authorized accounts
- **Liquidation Protection**: Automated liquidation prevents protocol insolvency


## License

This project is licensed under the ISC License.

## Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Pyth Network](https://pyth.network/)
- [Solana Program Library](https://spl.solana.com/)

## Disclaimer

This is experimental software. Use at your own risk. The protocol has not been audited and should not be used in production without proper security reviews.
