//! Tests for the real-world-asset tokenization generators and domain math.

use crate::{
    BASIS_POINTS_DENOMINATOR, CommodityToken, CommodityType, ContractGenerator,
    FractionalOwnership, IpAssetType, IpNft, Jurisdiction, OwnershipAllocation, PropertyType,
    RealEstateToken, RedemptionPolicy, RevenueDistributionFrequency, RevenueShareContract,
    TargetPlatform, distribute_revenue, price_per_share, validate_ownership_allocations,
};

fn alloc(holder: &str, basis_points: u32) -> OwnershipAllocation {
    OwnershipAllocation {
        holder: holder.to_string(),
        basis_points,
    }
}

fn real_estate_config() -> RealEstateToken {
    RealEstateToken {
        name: "MainStreetHoldings".to_string(),
        symbol: "MAIN".to_string(),
        property_type: PropertyType::Residential,
        property_identifier: "TITLE-0001".to_string(),
        appraised_value: 100_000_000,
        total_shares: 100_000,
        jurisdiction: Jurisdiction::Us,
        kyc_required: true,
        rental_income_enabled: true,
        transfer_lockup_days: 365,
        dividend_token: None,
    }
}

fn commodity_config() -> CommodityToken {
    CommodityToken {
        name: "VaultedGold".to_string(),
        symbol: "VGLD".to_string(),
        commodity_type: CommodityType::Gold,
        unit_label: "troy ounce".to_string(),
        backing_units_per_token: 1_000_000_000,
        custodian: "Acme Vaults Ltd".to_string(),
        redemption: RedemptionPolicy::Both,
        proof_of_reserves: true,
        jurisdiction: Jurisdiction::Ch,
    }
}

fn ip_config() -> IpNft {
    IpNft {
        name: "PatentPortfolio".to_string(),
        symbol: "PAT".to_string(),
        asset_type: IpAssetType::Patent,
        registration_number: "US-1234567".to_string(),
        jurisdiction: Jurisdiction::Us,
        licensing_enabled: true,
        royalty_basis_points: 500,
        royalty_receiver: "0x1111111111111111111111111111111111111111".to_string(),
        expiry_timestamp: Some(2_000_000_000),
        transferable: true,
    }
}

fn revenue_config() -> RevenueShareContract {
    RevenueShareContract {
        name: "SongRoyalties".to_string(),
        payment_token: None,
        allocations: vec![
            alloc("0x1111111111111111111111111111111111111111", 6000),
            alloc("0x2222222222222222222222222222222222222222", 4000),
        ],
        distribution_frequency: RevenueDistributionFrequency::Monthly,
        pull_payments: true,
        jurisdiction: Jurisdiction::Us,
    }
}

fn fractional_config() -> FractionalOwnership {
    FractionalOwnership {
        name: "FractionalApe".to_string(),
        symbol: "fAPE".to_string(),
        underlying_nft: "0x3333333333333333333333333333333333333333".to_string(),
        underlying_token_id: 42,
        total_fractions: 1_000_000,
        reserve_price: 5_000_000_000_000_000_000,
        curator: "0x4444444444444444444444444444444444444444".to_string(),
        curator_fee_basis_points: 250,
        buyout_enabled: true,
        jurisdiction: Jurisdiction::Eu,
    }
}

// --- Domain math: ownership allocation validation ------------------------------

#[test]
fn test_validate_allocations_accepts_exact_hundred_percent() {
    let allocations = [alloc("a", 2500), alloc("b", 2500), alloc("c", 5000)];
    assert!(validate_ownership_allocations(&allocations).is_ok());
}

#[test]
fn test_validate_allocations_rejects_empty() {
    assert!(validate_ownership_allocations(&[]).is_err());
}

#[test]
fn test_validate_allocations_rejects_wrong_sum() {
    let allocations = [alloc("a", 5000), alloc("b", 4000)];
    assert!(validate_ownership_allocations(&allocations).is_err());
}

#[test]
fn test_validate_allocations_rejects_zero_share() {
    let allocations = [alloc("a", 0), alloc("b", 10000)];
    assert!(validate_ownership_allocations(&allocations).is_err());
}

#[test]
fn test_validate_allocations_rejects_duplicate_holder() {
    let allocations = [alloc("dup", 5000), alloc("dup", 5000)];
    assert!(validate_ownership_allocations(&allocations).is_err());
}

// --- Domain math: revenue distribution ----------------------------------------

#[test]
fn test_distribute_revenue_is_proportional() {
    let allocations = [alloc("a", 2500), alloc("b", 7500)];
    let dist = distribute_revenue(1_000_000, &allocations).expect("valid distribution");
    assert_eq!(dist[0].amount, 250_000);
    assert_eq!(dist[1].amount, 750_000);
}

#[test]
fn test_distribute_revenue_is_dust_free_with_indivisible_split() {
    // 100 units across thirds is not evenly divisible; largest-remainder must
    // still preserve the exact total.
    let allocations = [alloc("a", 3333), alloc("b", 3333), alloc("c", 3334)];
    let dist = distribute_revenue(100, &allocations).expect("valid distribution");
    let total: u128 = dist.iter().map(|entry| entry.amount).sum();
    assert_eq!(total, 100);
    // The holder with the largest remainder (c) receives the extra unit.
    assert_eq!(dist[2].amount, 34);
    assert_eq!(dist[0].amount, 33);
    assert_eq!(dist[1].amount, 33);
}

#[test]
fn test_distribute_revenue_preserves_total_for_many_holders() {
    let allocations = [
        alloc("a", 1111),
        alloc("b", 2222),
        alloc("c", 3333),
        alloc("d", 3334),
    ];
    let dist = distribute_revenue(7, &allocations).expect("valid distribution");
    let total: u128 = dist.iter().map(|entry| entry.amount).sum();
    assert_eq!(total, 7);
}

#[test]
fn test_distribute_revenue_rejects_invalid_allocations() {
    let allocations = [alloc("a", 5000)];
    assert!(distribute_revenue(1000, &allocations).is_err());
}

#[test]
fn test_distribute_revenue_guards_overflow() {
    let allocations = [alloc("a", 10000)];
    assert!(distribute_revenue(u128::MAX, &allocations).is_err());
}

#[test]
fn test_price_per_share_floors_and_guards_zero() {
    assert_eq!(price_per_share(1005, 10).expect("non-zero shares"), 100);
    assert!(price_per_share(1000, 0).is_err());
}

#[test]
fn test_basis_points_denominator_constant() {
    assert_eq!(BASIS_POINTS_DENOMINATOR, 10_000);
}

// --- Real estate --------------------------------------------------------------

#[test]
fn test_real_estate_token_structure() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_real_estate_token(&real_estate_config())
        .expect("real estate generation");
    let src = &contract.source;
    assert_eq!(contract.name, "MainStreetHoldings");
    assert!(
        src.contains(
            "contract MainStreetHoldings is ERC20, Ownable2Step, Pausable, ReentrancyGuard"
        )
    );
    assert!(src.contains("function decimals() public pure override returns (uint8)"));
    assert!(src.contains("PRICE_PER_SHARE = 1000")); // 100_000_000 / 100_000
    assert!(src.contains("function withdrawDividend() external nonReentrant"));
    assert!(src.contains("magnifiedDividendPerShare"));
    // KYC + lock-up compliance hooks present.
    assert!(src.contains("kycApproved"));
    assert!(src.contains("TRANSFER_LOCKUP"));
    assert!(src.contains("RealEstate: sender in lock-up"));
}

#[test]
fn test_real_estate_token_with_erc20_dividends_uses_safe_erc20() {
    let mut config = real_estate_config();
    config.dividend_token = Some("0x5555555555555555555555555555555555555555".to_string());
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_real_estate_token(&config)
        .expect("real estate generation");
    assert!(contract.source.contains("using SafeERC20 for IERC20"));
    assert!(
        contract
            .source
            .contains("dividendToken.safeTransfer(msg.sender, amount)")
    );
    assert!(
        contract
            .source
            .contains("function distributeRentalIncome(uint256 amount) external onlyOwner")
    );
}

#[test]
fn test_real_estate_token_rejects_zero_shares() {
    let mut config = real_estate_config();
    config.total_shares = 0;
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    assert!(generator.generate_real_estate_token(&config).is_err());
}

// --- Commodity ----------------------------------------------------------------

#[test]
fn test_commodity_token_with_proof_of_reserves() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_commodity_token(&commodity_config())
        .expect("commodity generation");
    let src = &contract.source;
    assert!(src.contains("AggregatorV3Interface"));
    assert!(src.contains("constructor(address reserveFeed_)"));
    assert!(src.contains("insufficient reserves"));
    // Both redemption paths enabled.
    assert!(src.contains("function requestPhysicalRedemption"));
    assert!(src.contains("function redeemForCash"));
    assert!(src.contains("Each whole token is backed by 1000000000 nano-troy ounce"));
}

#[test]
fn test_commodity_token_non_redeemable_omits_redemption() {
    let mut config = commodity_config();
    config.redemption = RedemptionPolicy::NonRedeemable;
    config.proof_of_reserves = false;
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_commodity_token(&config)
        .expect("commodity generation");
    let src = &contract.source;
    assert!(!src.contains("function requestPhysicalRedemption"));
    assert!(!src.contains("function redeemForCash"));
    assert!(!src.contains("AggregatorV3Interface"));
    assert!(src.contains("constructor() ERC20"));
}

#[test]
fn test_commodity_token_custom_label_and_zero_backing() {
    let mut config = commodity_config();
    config.commodity_type = CommodityType::Custom("Lithium".to_string());
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_commodity_token(&config)
        .expect("commodity generation");
    assert!(contract.source.contains("custodied Lithium"));

    config.backing_units_per_token = 0;
    assert!(generator.generate_commodity_token(&config).is_err());
}

// --- IP NFT -------------------------------------------------------------------

#[test]
fn test_ip_nft_structure_and_royalties() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_ip_nft(&ip_config())
        .expect("ip generation");
    let src = &contract.source;
    assert!(src.contains("is ERC721, ERC721URIStorage, ERC2981, Ownable2Step"));
    assert!(src.contains("_setDefaultRoyalty(0x1111111111111111111111111111111111111111, 500)"));
    assert!(src.contains("function grantLicense"));
    assert!(src.contains("RIGHT_EXPIRY = 2000000000"));
    assert!(src.contains("require(block.timestamp < RIGHT_EXPIRY"));
    assert!(src.contains("override(ERC721, ERC721URIStorage, ERC2981)"));
}

#[test]
fn test_ip_nft_non_transferable_blocks_transfers() {
    let mut config = ip_config();
    config.transferable = false;
    config.licensing_enabled = false;
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator.generate_ip_nft(&config).expect("ip generation");
    let src = &contract.source;
    assert!(src.contains("IP: non-transferable"));
    assert!(src.contains("from == address(0) || to == address(0)"));
    assert!(!src.contains("function grantLicense"));
}

#[test]
fn test_ip_nft_rejects_excessive_royalty() {
    let mut config = ip_config();
    config.royalty_basis_points = 10_001;
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    assert!(generator.generate_ip_nft(&config).is_err());
}

// --- Revenue share ------------------------------------------------------------

#[test]
fn test_revenue_share_pull_payments_and_example() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let config = revenue_config();
    let contract = generator
        .generate_revenue_share(&config)
        .expect("revenue share generation");
    let src = &contract.source;
    assert!(src.contains("is ReentrancyGuard"));
    assert!(src.contains("function release(address account) external nonReentrant"));
    assert!(!src.contains("function distributeAll"));
    assert!(src.contains("_addShareholder(0x1111111111111111111111111111111111111111, 6000)"));
    // The worked example baked into NatSpec must match the domain math.
    let example = distribute_revenue(1_000_000, &config.allocations).expect("example");
    for entry in example {
        assert!(src.contains(&format!("- {}: {}", entry.holder, entry.amount)));
    }
}

#[test]
fn test_revenue_share_push_payments_with_erc20() {
    let mut config = revenue_config();
    config.pull_payments = false;
    config.payment_token = Some("0x6666666666666666666666666666666666666666".to_string());
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_revenue_share(&config)
        .expect("revenue share generation");
    let src = &contract.source;
    assert!(src.contains("function distributeAll() external nonReentrant"));
    assert!(src.contains("using SafeERC20 for IERC20"));
    assert!(src.contains("paymentToken.safeTransfer(account, payment)"));
    // No native receive() when distributing an ERC-20.
    assert!(!src.contains("receive() external payable"));
}

#[test]
fn test_revenue_share_rejects_invalid_allocations() {
    let mut config = revenue_config();
    config.allocations = vec![alloc("0x1111111111111111111111111111111111111111", 9000)];
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    assert!(generator.generate_revenue_share(&config).is_err());
}

// --- Fractional ownership -----------------------------------------------------

#[test]
fn test_fractional_ownership_with_buyout() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_fractional_ownership(&fractional_config())
        .expect("fractional generation");
    let src = &contract.source;
    assert!(src.contains("is ERC20, ERC721Holder, Ownable2Step, ReentrancyGuard"));
    assert!(src.contains("function activate() external onlyOwner"));
    assert!(src.contains("function redeemAll() external nonReentrant"));
    assert!(src.contains("function buyout() external payable nonReentrant"));
    assert!(src.contains("function redeemProceeds() external nonReentrant"));
    assert!(src.contains("CURATOR_FEE_BPS = 250"));
    assert!(src.contains("IMPLIED_FRACTION_PRICE = 5000000000000")); // reserve / fractions
}

#[test]
fn test_fractional_ownership_without_buyout_omits_buyout() {
    let mut config = fractional_config();
    config.buyout_enabled = false;
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator
        .generate_fractional_ownership(&config)
        .expect("fractional generation");
    let src = &contract.source;
    assert!(!src.contains("function buyout()"));
    assert!(!src.contains("function redeemProceeds()"));
    // Core vault redemption remains.
    assert!(src.contains("function redeemAll() external nonReentrant"));
}

#[test]
fn test_fractional_ownership_rejects_bad_inputs() {
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let mut zero_fractions = fractional_config();
    zero_fractions.total_fractions = 0;
    assert!(
        generator
            .generate_fractional_ownership(&zero_fractions)
            .is_err()
    );

    let mut bad_fee = fractional_config();
    bad_fee.curator_fee_basis_points = 10_001;
    assert!(generator.generate_fractional_ownership(&bad_fee).is_err());
}

// --- Multi-target composition -------------------------------------------------

#[test]
fn test_evm_l2_target_is_supported_and_preserved() {
    let generator = ContractGenerator::new(TargetPlatform::Base);
    let contract = generator
        .generate_real_estate_token(&real_estate_config())
        .expect("base generation");
    assert_eq!(contract.platform, TargetPlatform::Base);
    assert!(contract.source.contains("pragma solidity"));
}

#[test]
fn test_non_evm_targets_are_rejected() {
    for platform in [
        TargetPlatform::Move,
        TargetPlatform::Cairo,
        TargetPlatform::Solana,
    ] {
        let generator = ContractGenerator::new(platform);
        assert!(
            generator
                .generate_commodity_token(&commodity_config())
                .is_err()
        );
        assert!(generator.generate_ip_nft(&ip_config()).is_err());
        assert!(
            generator
                .generate_fractional_ownership(&fractional_config())
                .is_err()
        );
    }
}
