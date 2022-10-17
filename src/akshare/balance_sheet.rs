use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::Result;

use super::{shared::report_date_from_string, Akshare, ReportDate, StockCode};

impl Akshare {
    pub async fn get_balance_sheet(
        &self,
        symbol: &StockCode,
    ) -> Result<HashMap<ReportDate, BalanceSheetData>> {
        let data: Vec<BalanceSheetData> = self
            .transport
            .get("stock_balance_sheet_by_report_em", Some(symbol))
            .await?;
        let mut map = HashMap::with_capacity(data.len());
        for cash_data in data {
            map.insert(cash_data.report_date.to_owned(), cash_data);
        }
        Ok(map)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BalanceSheetData {
    /// 股票代码（全称）
    #[serde(rename = "SECUCODE")]
    pub secucode: String,
    /// 股票代码
    #[serde(rename = "SECURITY_CODE")]
    pub security_code: String,
    /// 股票名称
    #[serde(rename = "SECURITY_NAME_ABBR")]
    pub security_name_abbr: String,
    /// 证券组织代码
    #[serde(rename = "ORG_CODE")]
    pub org_code: String,
    /// 证券组织类型
    #[serde(rename = "ORG_TYPE")]
    pub org_type: String,
    /// 报告日期：0331/0630/0930/1231
    #[serde(rename = "REPORT_DATE", with = "report_date_from_string")]
    pub report_date: ReportDate,
    /// 报告类型
    #[serde(rename = "REPORT_TYPE")]
    pub report_type: String,
    /// 报告日期名字
    #[serde(rename = "REPORT_DATE_NAME")]
    pub report_date_name: Option<String>,
    ///
    #[serde(rename = "SECURITY_TYPE_CODE")]
    pub security_type_code: Option<String>,
    ///
    #[serde(rename = "NOTICE_DATE")]
    pub notice_date: Option<String>,
    ///
    #[serde(rename = "UPDATE_DATE")]
    pub update_date: Option<String>,
    ///
    #[serde(rename = "CURRENCY")]
    pub currency: Option<String>,
    ///
    #[serde(rename = "ACCEPT_DEPOSIT_INTERBANK")]
    pub accept_deposit_interbank: Option<f64>,
    /// 应付票据及应付账款
    #[serde(rename = "ACCOUNTS_PAYABLE")]
    pub accounts_payable: Option<f64>,
    /// 应收票据及应收账款
    #[serde(rename = "ACCOUNTS_RECE")]
    pub accounts_rece: Option<f64>,
    ///
    #[serde(rename = "ACCRUED_EXPENSE")]
    pub accrued_expense: Option<f64>,
    ///
    #[serde(rename = "ADVANCE_RECEIVABLES")]
    pub advance_receivables: Option<f64>,
    ///
    #[serde(rename = "AGENT_TRADE_SECURITY")]
    pub agent_trade_security: Option<f64>,
    ///
    #[serde(rename = "AGENT_UNDERWRITE_SECURITY")]
    pub agent_underwrite_security: Option<f64>,
    ///
    #[serde(rename = "AMORTIZE_COST_FINASSET")]
    pub amortize_cost_finasset: Option<f64>,
    ///
    #[serde(rename = "AMORTIZE_COST_FINLIAB")]
    pub amortize_cost_finliab: Option<f64>,
    ///
    #[serde(rename = "AMORTIZE_COST_NCFINASSET")]
    pub amortize_cost_ncfinasset: Option<f64>,
    ///
    #[serde(rename = "AMORTIZE_COST_NCFINLIAB")]
    pub amortize_cost_ncfinliab: Option<f64>,
    ///
    #[serde(rename = "APPOINT_FVTPL_FINASSET")]
    pub appoint_fvtpl_finasset: Option<f64>,
    ///
    #[serde(rename = "APPOINT_FVTPL_FINLIAB")]
    pub appoint_fvtpl_finliab: Option<f64>,
    ///
    #[serde(rename = "ASSET_BALANCE")]
    pub asset_balance: Option<f64>,
    ///
    #[serde(rename = "ASSET_OTHER")]
    pub asset_other: Option<f64>,
    ///
    #[serde(rename = "ASSIGN_CASH_DIVIDEND")]
    pub assign_cash_dividend: Option<f64>,
    ///
    #[serde(rename = "AVAILABLE_SALE_FINASSET")]
    pub available_sale_finasset: Option<f64>,
    ///
    #[serde(rename = "BOND_PAYABLE")]
    pub bond_payable: Option<f64>,
    ///
    #[serde(rename = "BORROW_FUND")]
    pub borrow_fund: Option<f64>,
    ///
    #[serde(rename = "BUY_RESALE_FINASSET")]
    pub buy_resale_finasset: Option<f64>,
    /// 资本公积
    #[serde(rename = "CAPITAL_RESERVE")]
    pub capital_reserve: Option<f64>,
    /// 在建工程
    #[serde(rename = "CIP")]
    pub cip: Option<f64>,
    ///
    #[serde(rename = "CONSUMPTIVE_BIOLOGICAL_ASSET")]
    pub consumptive_biological_asset: Option<f64>,
    /// 合同资产
    #[serde(rename = "CONTRACT_ASSET")]
    pub contract_asset: Option<f64>,
    /// 合同负债
    #[serde(rename = "CONTRACT_LIAB")]
    pub contract_liab: Option<f64>,
    ///
    #[serde(rename = "CONVERT_DIFF")]
    pub convert_diff: Option<f64>,
    ///
    #[serde(rename = "CREDITOR_INVEST")]
    pub creditor_invest: Option<f64>,
    ///
    #[serde(rename = "CURRENT_ASSET_BALANCE")]
    pub current_asset_balance: Option<f64>,
    ///
    #[serde(rename = "CURRENT_ASSET_OTHER")]
    pub current_asset_other: Option<f64>,
    ///
    #[serde(rename = "CURRENT_LIAB_BALANCE")]
    pub current_liab_balance: Option<f64>,
    ///
    #[serde(rename = "CURRENT_LIAB_OTHER")]
    pub current_liab_other: Option<f64>,
    ///
    #[serde(rename = "DEFER_INCOME")]
    pub defer_income: Option<f64>,
    ///
    #[serde(rename = "DEFER_INCOME_1YEAR")]
    pub defer_income_1year: Option<f64>,
    /// 递延所得税资产
    #[serde(rename = "DEFER_TAX_ASSET")]
    pub defer_tax_asset: Option<f64>,
    /// 递延所得税负债
    #[serde(rename = "DEFER_TAX_LIAB")]
    pub defer_tax_liab: Option<f64>,
    ///
    #[serde(rename = "DERIVE_FINASSET")]
    pub derive_finasset: Option<f64>,
    ///
    #[serde(rename = "DERIVE_FINLIAB")]
    pub derive_finliab: Option<f64>,
    ///
    #[serde(rename = "DEVELOP_EXPENSE")]
    pub develop_expense: Option<f64>,
    ///
    #[serde(rename = "DIV_HOLDSALE_ASSET")]
    pub div_holdsale_asset: Option<f64>,
    ///
    #[serde(rename = "DIV_HOLDSALE_LIAB")]
    pub div_holdsale_liab: Option<f64>,
    /// 其中:应付股利
    #[serde(rename = "DIVIDEND_PAYABLE")]
    pub dividend_payable: Option<f64>,
    ///
    #[serde(rename = "DIVIDEND_RECE")]
    pub dividend_rece: Option<f64>,
    ///
    #[serde(rename = "EQUITY_BALANCE")]
    pub equity_balance: Option<f64>,
    ///
    #[serde(rename = "EQUITY_OTHER")]
    pub equity_other: Option<f64>,
    ///
    #[serde(rename = "EXPORT_REFUND_RECE")]
    pub export_refund_rece: Option<f64>,
    ///
    #[serde(rename = "FEE_COMMISSION_PAYABLE")]
    pub fee_commission_payable: Option<f64>,
    ///
    #[serde(rename = "FIN_FUND")]
    pub fin_fund: Option<f64>,
    /// 应收款项融资
    #[serde(rename = "FINANCE_RECE")]
    pub finance_rece: Option<f64>,
    /// 固定资产
    #[serde(rename = "FIXED_ASSET")]
    pub fixed_asset: Option<f64>,
    ///
    #[serde(rename = "FIXED_ASSET_DISPOSAL")]
    pub fixed_asset_disposal: Option<f64>,
    ///
    #[serde(rename = "FVTOCI_FINASSET")]
    pub fvtoci_finasset: Option<f64>,
    ///
    #[serde(rename = "FVTOCI_NCFINASSET")]
    pub fvtoci_ncfinasset: Option<f64>,
    ///
    #[serde(rename = "FVTPL_FINASSET")]
    pub fvtpl_finasset: Option<f64>,
    ///
    #[serde(rename = "FVTPL_FINLIAB")]
    pub fvtpl_finliab: Option<f64>,
    ///
    #[serde(rename = "GENERAL_RISK_RESERVE")]
    pub general_risk_reserve: Option<f64>,
    /// 商誉
    #[serde(rename = "GOODWILL")]
    pub goodwill: Option<f64>,
    ///
    #[serde(rename = "HOLD_MATURITY_INVEST")]
    pub hold_maturity_invest: Option<f64>,
    ///
    #[serde(rename = "HOLDSALE_ASSET")]
    pub holdsale_asset: Option<f64>,
    ///
    #[serde(rename = "HOLDSALE_LIAB")]
    pub holdsale_liab: Option<f64>,
    ///
    #[serde(rename = "INSURANCE_CONTRACT_RESERVE")]
    pub insurance_contract_reserve: Option<f64>,
    /// 无形资产
    #[serde(rename = "INTANGIBLE_ASSET")]
    pub intangible_asset: Option<f64>,
    ///
    #[serde(rename = "INTEREST_PAYABLE")]
    pub interest_payable: Option<f64>,
    ///
    #[serde(rename = "INTEREST_RECE")]
    pub interest_rece: Option<f64>,
    ///
    #[serde(rename = "INTERNAL_PAYABLE")]
    pub internal_payable: Option<f64>,
    ///
    #[serde(rename = "INTERNAL_RECE")]
    pub internal_rece: Option<f64>,
    /// 存货
    #[serde(rename = "INVENTORY")]
    pub inventory: Option<f64>,
    ///
    #[serde(rename = "INVEST_REALESTATE")]
    pub invest_realestate: Option<f64>,
    /// 租赁负债
    #[serde(rename = "LEASE_LIAB")]
    pub lease_liab: Option<f64>,
    ///
    #[serde(rename = "LEND_FUND")]
    pub lend_fund: Option<f64>,
    ///
    #[serde(rename = "LIAB_BALANCE")]
    pub liab_balance: Option<f64>,
    ///
    #[serde(rename = "LIAB_EQUITY_BALANCE")]
    pub liab_equity_balance: Option<f64>,
    ///
    #[serde(rename = "LIAB_EQUITY_OTHER")]
    pub liab_equity_other: Option<f64>,
    ///
    #[serde(rename = "LIAB_OTHER")]
    pub liab_other: Option<f64>,
    ///
    #[serde(rename = "LOAN_ADVANCE")]
    pub loan_advance: Option<f64>,
    ///
    #[serde(rename = "LOAN_PBC")]
    pub loan_pbc: Option<f64>,
    /// 长期股权投资
    #[serde(rename = "LONG_EQUITY_INVEST")]
    pub long_equity_invest: Option<f64>,
    ///
    #[serde(rename = "LONG_LOAN")]
    pub long_loan: Option<f64>,
    ///
    #[serde(rename = "LONG_PAYABLE")]
    pub long_payable: Option<f64>,
    /// 长期待摊费用
    #[serde(rename = "LONG_PREPAID_EXPENSE")]
    pub long_prepaid_expense: Option<f64>,
    ///
    #[serde(rename = "LONG_RECE")]
    pub long_rece: Option<f64>,
    /// 长期应付职工薪酬
    #[serde(rename = "LONG_STAFFSALARY_PAYABLE")]
    pub long_staffsalary_payable: Option<f64>,
    /// 少数股东权益
    #[serde(rename = "MINORITY_EQUITY")]
    pub minority_equity: Option<f64>,
    /// 货币资金
    #[serde(rename = "MONETARYFUNDS")]
    pub monetaryfunds: Option<f64>,
    /// 一年内到期的非流动资产
    #[serde(rename = "NONCURRENT_ASSET_1YEAR")]
    pub noncurrent_asset_1year: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_ASSET_BALANCE")]
    pub noncurrent_asset_balance: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_ASSET_OTHER")]
    pub noncurrent_asset_other: Option<f64>,
    /// 一年内到期的非流动负债
    #[serde(rename = "NONCURRENT_LIAB_1YEAR")]
    pub noncurrent_liab_1year: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_LIAB_BALANCE")]
    pub noncurrent_liab_balance: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_LIAB_OTHER")]
    pub noncurrent_liab_other: Option<f64>,
    /// 其中:应付账款
    #[serde(rename = "NOTE_ACCOUNTS_PAYABLE")]
    pub note_accounts_payable: Option<f64>,
    /// 其中:应收账款
    #[serde(rename = "NOTE_ACCOUNTS_RECE")]
    pub note_accounts_rece: Option<f64>,
    ///
    #[serde(rename = "NOTE_PAYABLE")]
    pub note_payable: Option<f64>,
    ///
    #[serde(rename = "NOTE_RECE")]
    pub note_rece: Option<f64>,
    ///
    #[serde(rename = "OIL_GAS_ASSET")]
    pub oil_gas_asset: Option<f64>,
    /// 其他综合收益
    #[serde(rename = "OTHER_COMPRE_INCOME")]
    pub other_compre_income: Option<f64>,
    ///
    #[serde(rename = "OTHER_CREDITOR_INVEST")]
    pub other_creditor_invest: Option<f64>,
    /// 其他流动资产
    #[serde(rename = "OTHER_CURRENT_ASSET")]
    pub other_current_asset: Option<f64>,
    /// 其他流动负债
    #[serde(rename = "OTHER_CURRENT_LIAB")]
    pub other_current_liab: Option<f64>,
    /// 其他权益工具投资
    #[serde(rename = "OTHER_EQUITY_INVEST")]
    pub other_equity_invest: Option<f64>,
    ///
    #[serde(rename = "OTHER_EQUITY_OTHER")]
    pub other_equity_other: Option<f64>,
    ///
    #[serde(rename = "OTHER_EQUITY_TOOL")]
    pub other_equity_tool: Option<f64>,
    /// 其他非流动资产
    #[serde(rename = "OTHER_NONCURRENT_ASSET")]
    pub other_noncurrent_asset: Option<f64>,
    /// 其他非流动金融资产
    #[serde(rename = "OTHER_NONCURRENT_FINASSET")]
    pub other_noncurrent_finasset: Option<f64>,
    ///
    #[serde(rename = "OTHER_NONCURRENT_LIAB")]
    pub other_noncurrent_liab: Option<f64>,
    ///
    #[serde(rename = "OTHER_PAYABLE")]
    pub other_payable: Option<f64>,
    ///
    #[serde(rename = "OTHER_RECE")]
    pub other_rece: Option<f64>,
    ///
    #[serde(rename = "PARENT_EQUITY_BALANCE")]
    pub parent_equity_balance: Option<f64>,
    ///
    #[serde(rename = "PARENT_EQUITY_OTHER")]
    pub parent_equity_other: Option<f64>,
    ///
    #[serde(rename = "PERPETUAL_BOND")]
    pub perpetual_bond: Option<f64>,
    ///
    #[serde(rename = "PERPETUAL_BOND_PAYBALE")]
    pub perpetual_bond_paybale: Option<f64>,
    ///
    #[serde(rename = "PREDICT_CURRENT_LIAB")]
    pub predict_current_liab: Option<f64>,
    ///
    #[serde(rename = "PREDICT_LIAB")]
    pub predict_liab: Option<f64>,
    ///
    #[serde(rename = "PREFERRED_SHARES")]
    pub preferred_shares: Option<f64>,
    ///
    #[serde(rename = "PREFERRED_SHARES_PAYBALE")]
    pub preferred_shares_paybale: Option<f64>,
    ///
    #[serde(rename = "PREMIUM_RECE")]
    pub premium_rece: Option<f64>,
    /// 预付款项
    #[serde(rename = "PREPAYMENT")]
    pub prepayment: Option<f64>,
    ///
    #[serde(rename = "PRODUCTIVE_BIOLOGY_ASSET")]
    pub productive_biology_asset: Option<f64>,
    ///
    #[serde(rename = "PROJECT_MATERIAL")]
    pub project_material: Option<f64>,
    ///
    #[serde(rename = "RC_RESERVE_RECE")]
    pub rc_reserve_rece: Option<f64>,
    ///
    #[serde(rename = "REINSURE_PAYABLE")]
    pub reinsure_payable: Option<f64>,
    ///
    #[serde(rename = "REINSURE_RECE")]
    pub reinsure_rece: Option<f64>,
    ///
    #[serde(rename = "SELL_REPO_FINASSET")]
    pub sell_repo_finasset: Option<f64>,
    ///
    #[serde(rename = "SETTLE_EXCESS_RESERVE")]
    pub settle_excess_reserve: Option<f64>,
    /// 实收资本（或股本）
    #[serde(rename = "SHARE_CAPITAL")]
    pub share_capital: Option<f64>,
    ///
    #[serde(rename = "SHORT_BOND_PAYABLE")]
    pub short_bond_payable: Option<f64>,
    ///
    #[serde(rename = "SHORT_FIN_PAYABLE")]
    pub short_fin_payable: Option<f64>,
    /// 短期借款
    #[serde(rename = "SHORT_LOAN")]
    pub short_loan: Option<f64>,
    ///
    #[serde(rename = "SPECIAL_PAYABLE")]
    pub special_payable: Option<f64>,
    ///
    #[serde(rename = "SPECIAL_RESERVE")]
    pub special_reserve: Option<f64>,
    /// 应付职工薪酬
    #[serde(rename = "STAFF_SALARY_PAYABLE")]
    pub staff_salary_payable: Option<f64>,
    ///
    #[serde(rename = "SUBSIDY_RECE")]
    pub subsidy_rece: Option<f64>,
    ///
    #[serde(rename = "SURPLUS_RESERVE")]
    pub surplus_reserve: Option<f64>,
    /// 应交税费
    #[serde(rename = "TAX_PAYABLE")]
    pub tax_payable: Option<f64>,
    /// 资产总计
    #[serde(rename = "TOTAL_ASSETS")]
    pub total_assets: Option<f64>,
    /// 流动资产合计
    #[serde(rename = "TOTAL_CURRENT_ASSETS")]
    pub total_current_assets: Option<f64>,
    /// 流动负债合计
    #[serde(rename = "TOTAL_CURRENT_LIAB")]
    pub total_current_liab: Option<f64>,
    /// 股东权益合计
    #[serde(rename = "TOTAL_EQUITY")]
    pub total_equity: Option<f64>,
    /// 负债和股东权益总计
    #[serde(rename = "TOTAL_LIAB_EQUITY")]
    pub total_liab_equity: Option<f64>,
    /// 负债合计
    #[serde(rename = "TOTAL_LIABILITIES")]
    pub total_liabilities: Option<f64>,
    /// 非流动资产合计
    #[serde(rename = "TOTAL_NONCURRENT_ASSETS")]
    pub total_noncurrent_assets: Option<f64>,
    /// 非流动负债合计
    #[serde(rename = "TOTAL_NONCURRENT_LIAB")]
    pub total_noncurrent_liab: Option<f64>,
    /// 其他应付款合计
    #[serde(rename = "TOTAL_OTHER_PAYABLE")]
    pub total_other_payable: Option<f64>,
    /// 其他应收款合计
    #[serde(rename = "TOTAL_OTHER_RECE")]
    pub total_other_rece: Option<f64>,
    /// 归属于母公司股东权益总计
    #[serde(rename = "TOTAL_PARENT_EQUITY")]
    pub total_parent_equity: Option<f64>,
    ///
    #[serde(rename = "TRADE_FINASSET")]
    pub trade_finasset: Option<f64>,
    /// 交易性金融资产
    #[serde(rename = "TRADE_FINASSET_NOTFVTPL")]
    pub trade_finasset_notfvtpl: Option<f64>,
    ///
    #[serde(rename = "TRADE_FINLIAB")]
    pub trade_finliab: Option<f64>,
    ///
    #[serde(rename = "TRADE_FINLIAB_NOTFVTPL")]
    pub trade_finliab_notfvtpl: Option<f64>,
    ///
    #[serde(rename = "TREASURY_SHARES")]
    pub treasury_shares: Option<f64>,
    /// 未分配利润
    #[serde(rename = "UNASSIGN_RPOFIT")]
    pub unassign_rpofit: Option<f64>,
    ///
    #[serde(rename = "UNCONFIRM_INVEST_LOSS")]
    pub unconfirm_invest_loss: Option<f64>,
    /// 使用权资产
    #[serde(rename = "USERIGHT_ASSET")]
    pub useright_asset: Option<f64>,
    ///
    #[serde(rename = "ACCEPT_DEPOSIT_INTERBANK_YOY")]
    pub accept_deposit_interbank_yoy: Option<f64>,
    ///
    #[serde(rename = "ACCOUNTS_PAYABLE_YOY")]
    pub accounts_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "ACCOUNTS_RECE_YOY")]
    pub accounts_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "ACCRUED_EXPENSE_YOY")]
    pub accrued_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "ADVANCE_RECEIVABLES_YOY")]
    pub advance_receivables_yoy: Option<f64>,
    ///
    #[serde(rename = "AGENT_TRADE_SECURITY_YOY")]
    pub agent_trade_security_yoy: Option<f64>,
    ///
    #[serde(rename = "AGENT_UNDERWRITE_SECURITY_YOY")]
    pub agent_underwrite_security_yoy: Option<f64>,
    ///
    #[serde(rename = "AMORTIZE_COST_FINASSET_YOY")]
    pub amortize_cost_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "AMORTIZE_COST_FINLIAB_YOY")]
    pub amortize_cost_finliab_yoy: Option<f64>,
    ///
    #[serde(rename = "AMORTIZE_COST_NCFINASSET_YOY")]
    pub amortize_cost_ncfinasset_yoy: Option<f64>,
    ///
    #[serde(rename = "AMORTIZE_COST_NCFINLIAB_YOY")]
    pub amortize_cost_ncfinliab_yoy: Option<f64>,
    ///
    #[serde(rename = "APPOINT_FVTPL_FINASSET_YOY")]
    pub appoint_fvtpl_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "APPOINT_FVTPL_FINLIAB_YOY")]
    pub appoint_fvtpl_finliab_yoy: Option<f64>,
    ///
    #[serde(rename = "ASSET_BALANCE_YOY")]
    pub asset_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "ASSET_OTHER_YOY")]
    pub asset_other_yoy: Option<f64>,
    ///
    #[serde(rename = "ASSIGN_CASH_DIVIDEND_YOY")]
    pub assign_cash_dividend_yoy: Option<f64>,
    ///
    #[serde(rename = "AVAILABLE_SALE_FINASSET_YOY")]
    pub available_sale_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "BOND_PAYABLE_YOY")]
    pub bond_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "BORROW_FUND_YOY")]
    pub borrow_fund_yoy: Option<f64>,
    ///
    #[serde(rename = "BUY_RESALE_FINASSET_YOY")]
    pub buy_resale_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "CAPITAL_RESERVE_YOY")]
    pub capital_reserve_yoy: Option<f64>,
    ///
    #[serde(rename = "CIP_YOY")]
    pub cip_yoy: Option<f64>,
    ///
    #[serde(rename = "CONSUMPTIVE_BIOLOGICAL_ASSET_YOY")]
    pub consumptive_biological_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "CONTRACT_ASSET_YOY")]
    pub contract_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "CONTRACT_LIAB_YOY")]
    pub contract_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "CONVERT_DIFF_YOY")]
    pub convert_diff_yoy: Option<f64>,
    ///
    #[serde(rename = "CREDITOR_INVEST_YOY")]
    pub creditor_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "CURRENT_ASSET_BALANCE_YOY")]
    pub current_asset_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "CURRENT_ASSET_OTHER_YOY")]
    pub current_asset_other_yoy: Option<f64>,
    ///
    #[serde(rename = "CURRENT_LIAB_BALANCE_YOY")]
    pub current_liab_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "CURRENT_LIAB_OTHER_YOY")]
    pub current_liab_other_yoy: Option<f64>,
    ///
    #[serde(rename = "DEFER_INCOME_1YEAR_YOY")]
    pub defer_income_1year_yoy: Option<f64>,
    ///
    #[serde(rename = "DEFER_INCOME_YOY")]
    pub defer_income_yoy: Option<f64>,
    ///
    #[serde(rename = "DEFER_TAX_ASSET_YOY")]
    pub defer_tax_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "DEFER_TAX_LIAB_YOY")]
    pub defer_tax_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "DERIVE_FINASSET_YOY")]
    pub derive_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "DERIVE_FINLIAB_YOY")]
    pub derive_finliab_yoy: Option<f64>,
    ///
    #[serde(rename = "DEVELOP_EXPENSE_YOY")]
    pub develop_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "DIV_HOLDSALE_ASSET_YOY")]
    pub div_holdsale_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "DIV_HOLDSALE_LIAB_YOY")]
    pub div_holdsale_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "DIVIDEND_PAYABLE_YOY")]
    pub dividend_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "DIVIDEND_RECE_YOY")]
    pub dividend_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "EQUITY_BALANCE_YOY")]
    pub equity_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "EQUITY_OTHER_YOY")]
    pub equity_other_yoy: Option<f64>,
    ///
    #[serde(rename = "EXPORT_REFUND_RECE_YOY")]
    pub export_refund_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "FEE_COMMISSION_PAYABLE_YOY")]
    pub fee_commission_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "FIN_FUND_YOY")]
    pub fin_fund_yoy: Option<f64>,
    ///
    #[serde(rename = "FINANCE_RECE_YOY")]
    pub finance_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "FIXED_ASSET_DISPOSAL_YOY")]
    pub fixed_asset_disposal_yoy: Option<f64>,
    ///
    #[serde(rename = "FIXED_ASSET_YOY")]
    pub fixed_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "FVTOCI_FINASSET_YOY")]
    pub fvtoci_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "FVTOCI_NCFINASSET_YOY")]
    pub fvtoci_ncfinasset_yoy: Option<f64>,
    ///
    #[serde(rename = "FVTPL_FINASSET_YOY")]
    pub fvtpl_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "FVTPL_FINLIAB_YOY")]
    pub fvtpl_finliab_yoy: Option<f64>,
    ///
    #[serde(rename = "GENERAL_RISK_RESERVE_YOY")]
    pub general_risk_reserve_yoy: Option<f64>,
    ///
    #[serde(rename = "GOODWILL_YOY")]
    pub goodwill_yoy: Option<f64>,
    ///
    #[serde(rename = "HOLD_MATURITY_INVEST_YOY")]
    pub hold_maturity_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "HOLDSALE_ASSET_YOY")]
    pub holdsale_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "HOLDSALE_LIAB_YOY")]
    pub holdsale_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "INSURANCE_CONTRACT_RESERVE_YOY")]
    pub insurance_contract_reserve_yoy: Option<f64>,
    ///
    #[serde(rename = "INTANGIBLE_ASSET_YOY")]
    pub intangible_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "INTEREST_PAYABLE_YOY")]
    pub interest_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "INTEREST_RECE_YOY")]
    pub interest_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "INTERNAL_PAYABLE_YOY")]
    pub internal_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "INTERNAL_RECE_YOY")]
    pub internal_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "INVENTORY_YOY")]
    pub inventory_yoy: Option<f64>,
    ///
    #[serde(rename = "INVEST_REALESTATE_YOY")]
    pub invest_realestate_yoy: Option<f64>,
    ///
    #[serde(rename = "LEASE_LIAB_YOY")]
    pub lease_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "LEND_FUND_YOY")]
    pub lend_fund_yoy: Option<f64>,
    ///
    #[serde(rename = "LIAB_BALANCE_YOY")]
    pub liab_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "LIAB_EQUITY_BALANCE_YOY")]
    pub liab_equity_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "LIAB_EQUITY_OTHER_YOY")]
    pub liab_equity_other_yoy: Option<f64>,
    ///
    #[serde(rename = "LIAB_OTHER_YOY")]
    pub liab_other_yoy: Option<f64>,
    ///
    #[serde(rename = "LOAN_ADVANCE_YOY")]
    pub loan_advance_yoy: Option<f64>,
    ///
    #[serde(rename = "LOAN_PBC_YOY")]
    pub loan_pbc_yoy: Option<f64>,
    ///
    #[serde(rename = "LONG_EQUITY_INVEST_YOY")]
    pub long_equity_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "LONG_LOAN_YOY")]
    pub long_loan_yoy: Option<f64>,
    ///
    #[serde(rename = "LONG_PAYABLE_YOY")]
    pub long_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "LONG_PREPAID_EXPENSE_YOY")]
    pub long_prepaid_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "LONG_RECE_YOY")]
    pub long_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "LONG_STAFFSALARY_PAYABLE_YOY")]
    pub long_staffsalary_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "MINORITY_EQUITY_YOY")]
    pub minority_equity_yoy: Option<f64>,
    ///
    #[serde(rename = "MONETARYFUNDS_YOY")]
    pub monetaryfunds_yoy: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_ASSET_1YEAR_YOY")]
    pub noncurrent_asset_1year_yoy: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_ASSET_BALANCE_YOY")]
    pub noncurrent_asset_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_ASSET_OTHER_YOY")]
    pub noncurrent_asset_other_yoy: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_LIAB_1YEAR_YOY")]
    pub noncurrent_liab_1year_yoy: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_LIAB_BALANCE_YOY")]
    pub noncurrent_liab_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_LIAB_OTHER_YOY")]
    pub noncurrent_liab_other_yoy: Option<f64>,
    ///
    #[serde(rename = "NOTE_ACCOUNTS_PAYABLE_YOY")]
    pub note_accounts_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "NOTE_ACCOUNTS_RECE_YOY")]
    pub note_accounts_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "NOTE_PAYABLE_YOY")]
    pub note_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "NOTE_RECE_YOY")]
    pub note_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "OIL_GAS_ASSET_YOY")]
    pub oil_gas_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_COMPRE_INCOME_YOY")]
    pub other_compre_income_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_CREDITOR_INVEST_YOY")]
    pub other_creditor_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_CURRENT_ASSET_YOY")]
    pub other_current_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_CURRENT_LIAB_YOY")]
    pub other_current_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_EQUITY_INVEST_YOY")]
    pub other_equity_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_EQUITY_OTHER_YOY")]
    pub other_equity_other_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_EQUITY_TOOL_YOY")]
    pub other_equity_tool_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_NONCURRENT_ASSET_YOY")]
    pub other_noncurrent_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_NONCURRENT_FINASSET_YOY")]
    pub other_noncurrent_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_NONCURRENT_LIAB_YOY")]
    pub other_noncurrent_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_PAYABLE_YOY")]
    pub other_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_RECE_YOY")]
    pub other_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "PARENT_EQUITY_BALANCE_YOY")]
    pub parent_equity_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "PARENT_EQUITY_OTHER_YOY")]
    pub parent_equity_other_yoy: Option<f64>,
    ///
    #[serde(rename = "PERPETUAL_BOND_PAYBALE_YOY")]
    pub perpetual_bond_paybale_yoy: Option<f64>,
    ///
    #[serde(rename = "PERPETUAL_BOND_YOY")]
    pub perpetual_bond_yoy: Option<f64>,
    ///
    #[serde(rename = "PREDICT_CURRENT_LIAB_YOY")]
    pub predict_current_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "PREDICT_LIAB_YOY")]
    pub predict_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "PREFERRED_SHARES_PAYBALE_YOY")]
    pub preferred_shares_paybale_yoy: Option<f64>,
    ///
    #[serde(rename = "PREFERRED_SHARES_YOY")]
    pub preferred_shares_yoy: Option<f64>,
    ///
    #[serde(rename = "PREMIUM_RECE_YOY")]
    pub premium_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "PREPAYMENT_YOY")]
    pub prepayment_yoy: Option<f64>,
    ///
    #[serde(rename = "PRODUCTIVE_BIOLOGY_ASSET_YOY")]
    pub productive_biology_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "PROJECT_MATERIAL_YOY")]
    pub project_material_yoy: Option<f64>,
    ///
    #[serde(rename = "RC_RESERVE_RECE_YOY")]
    pub rc_reserve_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "REINSURE_PAYABLE_YOY")]
    pub reinsure_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "REINSURE_RECE_YOY")]
    pub reinsure_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "SELL_REPO_FINASSET_YOY")]
    pub sell_repo_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "SETTLE_EXCESS_RESERVE_YOY")]
    pub settle_excess_reserve_yoy: Option<f64>,
    ///
    #[serde(rename = "SHARE_CAPITAL_YOY")]
    pub share_capital_yoy: Option<f64>,
    ///
    #[serde(rename = "SHORT_BOND_PAYABLE_YOY")]
    pub short_bond_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "SHORT_FIN_PAYABLE_YOY")]
    pub short_fin_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "SHORT_LOAN_YOY")]
    pub short_loan_yoy: Option<f64>,
    ///
    #[serde(rename = "SPECIAL_PAYABLE_YOY")]
    pub special_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "SPECIAL_RESERVE_YOY")]
    pub special_reserve_yoy: Option<f64>,
    ///
    #[serde(rename = "STAFF_SALARY_PAYABLE_YOY")]
    pub staff_salary_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "SUBSIDY_RECE_YOY")]
    pub subsidy_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "SURPLUS_RESERVE_YOY")]
    pub surplus_reserve_yoy: Option<f64>,
    ///
    #[serde(rename = "TAX_PAYABLE_YOY")]
    pub tax_payable_yoy: Option<f64>,
    /// 总资产同比
    #[serde(rename = "TOTAL_ASSETS_YOY")]
    pub total_assets_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_CURRENT_ASSETS_YOY")]
    pub total_current_assets_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_CURRENT_LIAB_YOY")]
    pub total_current_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_EQUITY_YOY")]
    pub total_equity_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_LIAB_EQUITY_YOY")]
    pub total_liab_equity_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_LIABILITIES_YOY")]
    pub total_liabilities_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_NONCURRENT_ASSETS_YOY")]
    pub total_noncurrent_assets_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_NONCURRENT_LIAB_YOY")]
    pub total_noncurrent_liab_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_OTHER_PAYABLE_YOY")]
    pub total_other_payable_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_OTHER_RECE_YOY")]
    pub total_other_rece_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_PARENT_EQUITY_YOY")]
    pub total_parent_equity_yoy: Option<f64>,
    ///
    #[serde(rename = "TRADE_FINASSET_NOTFVTPL_YOY")]
    pub trade_finasset_notfvtpl_yoy: Option<f64>,
    ///
    #[serde(rename = "TRADE_FINASSET_YOY")]
    pub trade_finasset_yoy: Option<f64>,
    ///
    #[serde(rename = "TRADE_FINLIAB_NOTFVTPL_YOY")]
    pub trade_finliab_notfvtpl_yoy: Option<f64>,
    ///
    #[serde(rename = "TRADE_FINLIAB_YOY")]
    pub trade_finliab_yoy: Option<f64>,
    ///
    #[serde(rename = "TREASURY_SHARES_YOY")]
    pub treasury_shares_yoy: Option<f64>,
    ///
    #[serde(rename = "UNASSIGN_RPOFIT_YOY")]
    pub unassign_rpofit_yoy: Option<f64>,
    ///
    #[serde(rename = "UNCONFIRM_INVEST_LOSS_YOY")]
    pub unconfirm_invest_loss_yoy: Option<f64>,
    ///
    #[serde(rename = "USERIGHT_ASSET_YOY")]
    pub useright_asset_yoy: Option<f64>,
    /// 审计意见
    #[serde(rename = "OPINION_TYPE")]
    pub opinion_type: Option<String>,
    ///
    #[serde(rename = "OSOPINION_TYPE")]
    pub osopinion_type: Option<String>,
    ///
    #[serde(rename = "LISTING_STATE")]
    pub listing_state: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let akshare = Akshare::new();
        if let Ok(client) = akshare {
            let s = client.get_balance_sheet(&"SZ002027".into()).await.unwrap();
            print!(
                "{:?}",
                s.get(&ReportDate::new(2022, crate::akshare::Quarter::Q1))
            );
        }
    }
}
