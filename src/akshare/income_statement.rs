use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::Result;

use super::{Akshare, StockCode};

impl Akshare {
    pub async fn get_income_statement(
        &self,
        symbol: &StockCode,
    ) -> Result<HashMap<String, IncomeStatementData>> {
        let data: Vec<IncomeStatementData> = self
            .transport
            .get("stock_profit_sheet_by_report_em", Some(symbol))
            .await?;
        let mut map = HashMap::with_capacity(data.len());
        for cash_data in data {
            map.insert(cash_data.report_date.to_owned(), cash_data);
        }
        Ok(map)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IncomeStatementData {
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
    #[serde(rename = "REPORT_DATE")]
    pub report_date: String,
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
    /// 营业总收入
    #[serde(rename = "TOTAL_OPERATE_INCOME")]
    pub total_operate_income: Option<f64>,
    ///
    #[serde(rename = "TOTAL_OPERATE_INCOME_YOY")]
    pub total_operate_income_yoy: Option<f64>,
    /// 营业收入
    #[serde(rename = "OPERATE_INCOME")]
    pub operate_income: Option<f64>,
    ///
    #[serde(rename = "OPERATE_INCOME_YOY")]
    pub operate_income_yoy: Option<f64>,
    ///
    #[serde(rename = "INTEREST_INCOME")]
    pub interest_income: Option<f64>,
    ///
    #[serde(rename = "INTEREST_INCOME_YOY")]
    pub interest_income_yoy: Option<f64>,
    ///
    #[serde(rename = "EARNED_PREMIUM")]
    pub earned_premium: Option<f64>,
    ///
    #[serde(rename = "EARNED_PREMIUM_YOY")]
    pub earned_premium_yoy: Option<f64>,
    ///
    #[serde(rename = "FEE_COMMISSION_INCOME")]
    pub fee_commission_income: Option<f64>,
    ///
    #[serde(rename = "FEE_COMMISSION_INCOME_YOY")]
    pub fee_commission_income_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_BUSINESS_INCOME")]
    pub other_business_income: Option<f64>,
    ///
    #[serde(rename = "OTHER_BUSINESS_INCOME_YOY")]
    pub other_business_income_yoy: Option<f64>,
    ///
    #[serde(rename = "TOI_OTHER")]
    pub toi_other: Option<f64>,
    ///
    #[serde(rename = "TOI_OTHER_YOY")]
    pub toi_other_yoy: Option<f64>,
    /// 营业总成本
    #[serde(rename = "TOTAL_OPERATE_COST")]
    pub total_operate_cost: Option<f64>,
    ///
    #[serde(rename = "TOTAL_OPERATE_COST_YOY")]
    pub total_operate_cost_yoy: Option<f64>,
    /// 营业成本
    #[serde(rename = "OPERATE_COST")]
    pub operate_cost: Option<f64>,
    ///
    #[serde(rename = "OPERATE_COST_YOY")]
    pub operate_cost_yoy: Option<f64>,
    ///
    #[serde(rename = "INTEREST_EXPENSE")]
    pub interest_expense: Option<f64>,
    ///
    #[serde(rename = "INTEREST_EXPENSE_YOY")]
    pub interest_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "FEE_COMMISSION_EXPENSE")]
    pub fee_commission_expense: Option<f64>,
    ///
    #[serde(rename = "FEE_COMMISSION_EXPENSE_YOY")]
    pub fee_commission_expense_yoy: Option<f64>,
    /// 研发费用
    #[serde(rename = "RESEARCH_EXPENSE")]
    pub research_expense: Option<f64>,
    ///
    #[serde(rename = "RESEARCH_EXPENSE_YOY")]
    pub research_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "SURRENDER_VALUE")]
    pub surrender_value: Option<f64>,
    ///
    #[serde(rename = "SURRENDER_VALUE_YOY")]
    pub surrender_value_yoy: Option<f64>,
    ///
    #[serde(rename = "NET_COMPENSATE_EXPENSE")]
    pub net_compensate_expense: Option<f64>,
    ///
    #[serde(rename = "NET_COMPENSATE_EXPENSE_YOY")]
    pub net_compensate_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "NET_CONTRACT_RESERVE")]
    pub net_contract_reserve: Option<f64>,
    ///
    #[serde(rename = "NET_CONTRACT_RESERVE_YOY")]
    pub net_contract_reserve_yoy: Option<f64>,
    ///
    #[serde(rename = "POLICY_BONUS_EXPENSE")]
    pub policy_bonus_expense: Option<f64>,
    ///
    #[serde(rename = "POLICY_BONUS_EXPENSE_YOY")]
    pub policy_bonus_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "REINSURE_EXPENSE")]
    pub reinsure_expense: Option<f64>,
    ///
    #[serde(rename = "REINSURE_EXPENSE_YOY")]
    pub reinsure_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_BUSINESS_COST")]
    pub other_business_cost: Option<f64>,
    ///
    #[serde(rename = "OTHER_BUSINESS_COST_YOY")]
    pub other_business_cost_yoy: Option<f64>,
    /// 税金及附加
    #[serde(rename = "OPERATE_TAX_ADD")]
    pub operate_tax_add: Option<f64>,
    ///
    #[serde(rename = "OPERATE_TAX_ADD_YOY")]
    pub operate_tax_add_yoy: Option<f64>,
    /// 销售费用
    #[serde(rename = "SALE_EXPENSE")]
    pub sale_expense: Option<f64>,
    ///
    #[serde(rename = "SALE_EXPENSE_YOY")]
    pub sale_expense_yoy: Option<f64>,
    /// 管理费用
    #[serde(rename = "MANAGE_EXPENSE")]
    pub manage_expense: Option<f64>,
    ///
    #[serde(rename = "MANAGE_EXPENSE_YOY")]
    pub manage_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "ME_RESEARCH_EXPENSE")]
    pub me_research_expense: Option<f64>,
    ///
    #[serde(rename = "ME_RESEARCH_EXPENSE_YOY")]
    pub me_research_expense_yoy: Option<f64>,
    /// 财务费用
    #[serde(rename = "FINANCE_EXPENSE")]
    pub finance_expense: Option<f64>,
    ///
    #[serde(rename = "FINANCE_EXPENSE_YOY")]
    pub finance_expense_yoy: Option<f64>,
    /// 其中:利息费用
    #[serde(rename = "FE_INTEREST_EXPENSE")]
    pub fe_interest_expense: Option<f64>,
    ///
    #[serde(rename = "FE_INTEREST_EXPENSE_YOY")]
    pub fe_interest_expense_yoy: Option<f64>,
    /// 利息收入
    #[serde(rename = "FE_INTEREST_INCOME")]
    pub fe_interest_income: Option<f64>,
    ///
    #[serde(rename = "FE_INTEREST_INCOME_YOY")]
    pub fe_interest_income_yoy: Option<f64>,
    ///
    #[serde(rename = "ASSET_IMPAIRMENT_LOSS")]
    pub asset_impairment_loss: Option<f64>,
    ///
    #[serde(rename = "ASSET_IMPAIRMENT_LOSS_YOY")]
    pub asset_impairment_loss_yoy: Option<f64>,
    ///
    #[serde(rename = "CREDIT_IMPAIRMENT_LOSS")]
    pub credit_impairment_loss: Option<f64>,
    ///
    #[serde(rename = "CREDIT_IMPAIRMENT_LOSS_YOY")]
    pub credit_impairment_loss_yoy: Option<f64>,
    ///
    #[serde(rename = "TOC_OTHER")]
    pub toc_other: Option<f64>,
    ///
    #[serde(rename = "TOC_OTHER_YOY")]
    pub toc_other_yoy: Option<f64>,
    /// 加:公允价值变动收益
    #[serde(rename = "FAIRVALUE_CHANGE_INCOME")]
    pub fairvalue_change_income: Option<f64>,
    ///
    #[serde(rename = "FAIRVALUE_CHANGE_INCOME_YOY")]
    pub fairvalue_change_income_yoy: Option<f64>,
    /// 投资收益
    #[serde(rename = "INVEST_INCOME")]
    pub invest_income: Option<f64>,
    ///
    #[serde(rename = "INVEST_INCOME_YOY")]
    pub invest_income_yoy: Option<f64>,
    /// 其中:对联营企业和合营企业的投资收益
    #[serde(rename = "INVEST_JOINT_INCOME")]
    pub invest_joint_income: Option<f64>,
    ///
    #[serde(rename = "INVEST_JOINT_INCOME_YOY")]
    pub invest_joint_income_yoy: Option<f64>,
    ///
    #[serde(rename = "NET_EXPOSURE_INCOME")]
    pub net_exposure_income: Option<f64>,
    ///
    #[serde(rename = "NET_EXPOSURE_INCOME_YOY")]
    pub net_exposure_income_yoy: Option<f64>,
    ///
    #[serde(rename = "EXCHANGE_INCOME")]
    pub exchange_income: Option<f64>,
    ///
    #[serde(rename = "EXCHANGE_INCOME_YOY")]
    pub exchange_income_yoy: Option<f64>,
    /// 资产处置收益
    #[serde(rename = "ASSET_DISPOSAL_INCOME")]
    pub asset_disposal_income: Option<f64>,
    ///
    #[serde(rename = "ASSET_DISPOSAL_INCOME_YOY")]
    pub asset_disposal_income_yoy: Option<f64>,
    /// 资产减值损失(新)
    #[serde(rename = "ASSET_IMPAIRMENT_INCOME")]
    pub asset_impairment_income: Option<f64>,
    ///
    #[serde(rename = "ASSET_IMPAIRMENT_INCOME_YOY")]
    pub asset_impairment_income_yoy: Option<f64>,
    /// 信用减值损失(新)
    #[serde(rename = "CREDIT_IMPAIRMENT_INCOME")]
    pub credit_impairment_income: Option<f64>,
    ///
    #[serde(rename = "CREDIT_IMPAIRMENT_INCOME_YOY")]
    pub credit_impairment_income_yoy: Option<f64>,
    /// 其他收益
    #[serde(rename = "OTHER_INCOME")]
    pub other_income: Option<f64>,
    ///
    #[serde(rename = "OTHER_INCOME_YOY")]
    pub other_income_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_PROFIT_OTHER")]
    pub operate_profit_other: Option<f64>,
    ///
    #[serde(rename = "OPERATE_PROFIT_OTHER_YOY")]
    pub operate_profit_other_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_PROFIT_BALANCE")]
    pub operate_profit_balance: Option<f64>,
    ///
    #[serde(rename = "OPERATE_PROFIT_BALANCE_YOY")]
    pub operate_profit_balance_yoy: Option<f64>,
    /// 营业利润
    #[serde(rename = "OPERATE_PROFIT")]
    pub operate_profit: Option<f64>,
    ///
    #[serde(rename = "OPERATE_PROFIT_YOY")]
    pub operate_profit_yoy: Option<f64>,
    /// 加:营业外收入
    #[serde(rename = "NONBUSINESS_INCOME")]
    pub nonbusiness_income: Option<f64>,
    ///
    #[serde(rename = "NONBUSINESS_INCOME_YOY")]
    pub nonbusiness_income_yoy: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_DISPOSAL_INCOME")]
    pub noncurrent_disposal_income: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_DISPOSAL_INCOME_YOY")]
    pub noncurrent_disposal_income_yoy: Option<f64>,
    /// 减:营业外支出
    #[serde(rename = "NONBUSINESS_EXPENSE")]
    pub nonbusiness_expense: Option<f64>,
    ///
    #[serde(rename = "NONBUSINESS_EXPENSE_YOY")]
    pub nonbusiness_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_DISPOSAL_LOSS")]
    pub noncurrent_disposal_loss: Option<f64>,
    ///
    #[serde(rename = "NONCURRENT_DISPOSAL_LOSS_YOY")]
    pub noncurrent_disposal_loss_yoy: Option<f64>,
    ///
    #[serde(rename = "EFFECT_TP_OTHER")]
    pub effect_tp_other: Option<f64>,
    ///
    #[serde(rename = "EFFECT_TP_OTHER_YOY")]
    pub effect_tp_other_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_PROFIT_BALANCE")]
    pub total_profit_balance: Option<f64>,
    ///
    #[serde(rename = "TOTAL_PROFIT_BALANCE_YOY")]
    pub total_profit_balance_yoy: Option<f64>,
    /// 利润总额
    #[serde(rename = "TOTAL_PROFIT")]
    pub total_profit: Option<f64>,
    ///
    #[serde(rename = "TOTAL_PROFIT_YOY")]
    pub total_profit_yoy: Option<f64>,
    /// 减:所得税
    #[serde(rename = "INCOME_TAX")]
    pub income_tax: Option<f64>,
    ///
    #[serde(rename = "INCOME_TAX_YOY")]
    pub income_tax_yoy: Option<f64>,
    ///
    #[serde(rename = "EFFECT_NETPROFIT_OTHER")]
    pub effect_netprofit_other: Option<f64>,
    ///
    #[serde(rename = "EFFECT_NETPROFIT_OTHER_YOY")]
    pub effect_netprofit_other_yoy: Option<f64>,
    ///
    #[serde(rename = "EFFECT_NETPROFIT_BALANCE")]
    pub effect_netprofit_balance: Option<f64>,
    ///
    #[serde(rename = "EFFECT_NETPROFIT_BALANCE_YOY")]
    pub effect_netprofit_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "UNCONFIRM_INVEST_LOSS")]
    pub unconfirm_invest_loss: Option<f64>,
    ///
    #[serde(rename = "UNCONFIRM_INVEST_LOSS_YOY")]
    pub unconfirm_invest_loss_yoy: Option<f64>,
    /// 净利润
    #[serde(rename = "NETPROFIT")]
    pub netprofit: Option<f64>,
    ///
    #[serde(rename = "NETPROFIT_YOY")]
    pub netprofit_yoy: Option<f64>,
    ///
    #[serde(rename = "PRECOMBINE_PROFIT")]
    pub precombine_profit: Option<f64>,
    ///
    #[serde(rename = "PRECOMBINE_PROFIT_YOY")]
    pub precombine_profit_yoy: Option<f64>,
    /// (一)按经营持续性分类  持续经营净利润
    #[serde(rename = "CONTINUED_NETPROFIT")]
    pub continued_netprofit: Option<f64>,
    ///
    #[serde(rename = "CONTINUED_NETPROFIT_YOY")]
    pub continued_netprofit_yoy: Option<f64>,
    ///
    #[serde(rename = "DISCONTINUED_NETPROFIT")]
    pub discontinued_netprofit: Option<f64>,
    ///
    #[serde(rename = "DISCONTINUED_NETPROFIT_YOY")]
    pub discontinued_netprofit_yoy: Option<f64>,
    /// (二)按所有权归属分类  归属于母公司股东的净利润
    #[serde(rename = "PARENT_NETPROFIT")]
    pub parent_netprofit: Option<f64>,
    ///
    #[serde(rename = "PARENT_NETPROFIT_YOY")]
    pub parent_netprofit_yoy: Option<f64>,
    /// (二)按所有权归属分类  少数股东损益
    #[serde(rename = "MINORITY_INTEREST")]
    pub minority_interest: Option<f64>,
    ///
    #[serde(rename = "MINORITY_INTEREST_YOY")]
    pub minority_interest_yoy: Option<f64>,
    /// (二)按所有权归属分类  扣除非经常性损益后的净利润
    #[serde(rename = "DEDUCT_PARENT_NETPROFIT")]
    pub deduct_parent_netprofit: Option<f64>,
    ///
    #[serde(rename = "DEDUCT_PARENT_NETPROFIT_YOY")]
    pub deduct_parent_netprofit_yoy: Option<f64>,
    ///
    #[serde(rename = "NETPROFIT_OTHER")]
    pub netprofit_other: Option<f64>,
    ///
    #[serde(rename = "NETPROFIT_OTHER_YOY")]
    pub netprofit_other_yoy: Option<f64>,
    ///
    #[serde(rename = "NETPROFIT_BALANCE")]
    pub netprofit_balance: Option<f64>,
    ///
    #[serde(rename = "NETPROFIT_BALANCE_YOY")]
    pub netprofit_balance_yoy: Option<f64>,
    /// 每股收益  基本每股收益
    #[serde(rename = "BASIC_EPS")]
    pub basic_eps: Option<f64>,
    ///
    #[serde(rename = "BASIC_EPS_YOY")]
    pub basic_eps_yoy: Option<f64>,
    /// 每股收益  稀释每股收益
    #[serde(rename = "DILUTED_EPS")]
    pub diluted_eps: Option<f64>,
    ///
    #[serde(rename = "DILUTED_EPS_YOY")]
    pub diluted_eps_yoy: Option<f64>,
    /// 其他综合收益
    #[serde(rename = "OTHER_COMPRE_INCOME")]
    pub other_compre_income: Option<f64>,
    ///
    #[serde(rename = "OTHER_COMPRE_INCOME_YOY")]
    pub other_compre_income_yoy: Option<f64>,
    /// 其他综合收益  归属于母公司股东的其他综合收益
    #[serde(rename = "PARENT_OCI")]
    pub parent_oci: Option<f64>,
    ///
    #[serde(rename = "PARENT_OCI_YOY")]
    pub parent_oci_yoy: Option<f64>,
    /// 其他综合收益  归属于少数股东的其他综合收益
    #[serde(rename = "MINORITY_OCI")]
    pub minority_oci: Option<f64>,
    ///
    #[serde(rename = "MINORITY_OCI_YOY")]
    pub minority_oci_yoy: Option<f64>,
    ///
    #[serde(rename = "PARENT_OCI_OTHER")]
    pub parent_oci_other: Option<f64>,
    ///
    #[serde(rename = "PARENT_OCI_OTHER_YOY")]
    pub parent_oci_other_yoy: Option<f64>,
    ///
    #[serde(rename = "PARENT_OCI_BALANCE")]
    pub parent_oci_balance: Option<f64>,
    ///
    #[serde(rename = "PARENT_OCI_BALANCE_YOY")]
    pub parent_oci_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "UNABLE_OCI")]
    pub unable_oci: Option<f64>,
    ///
    #[serde(rename = "UNABLE_OCI_YOY")]
    pub unable_oci_yoy: Option<f64>,
    ///
    #[serde(rename = "CREDITRISK_FAIRVALUE_CHANGE")]
    pub creditrisk_fairvalue_change: Option<f64>,
    ///
    #[serde(rename = "CREDITRISK_FAIRVALUE_CHANGE_YOY")]
    pub creditrisk_fairvalue_change_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHERRIGHT_FAIRVALUE_CHANGE")]
    pub otherright_fairvalue_change: Option<f64>,
    ///
    #[serde(rename = "OTHERRIGHT_FAIRVALUE_CHANGE_YOY")]
    pub otherright_fairvalue_change_yoy: Option<f64>,
    ///
    #[serde(rename = "SETUP_PROFIT_CHANGE")]
    pub setup_profit_change: Option<f64>,
    ///
    #[serde(rename = "SETUP_PROFIT_CHANGE_YOY")]
    pub setup_profit_change_yoy: Option<f64>,
    ///
    #[serde(rename = "RIGHTLAW_UNABLE_OCI")]
    pub rightlaw_unable_oci: Option<f64>,
    ///
    #[serde(rename = "RIGHTLAW_UNABLE_OCI_YOY")]
    pub rightlaw_unable_oci_yoy: Option<f64>,
    ///
    #[serde(rename = "UNABLE_OCI_OTHER")]
    pub unable_oci_other: Option<f64>,
    ///
    #[serde(rename = "UNABLE_OCI_OTHER_YOY")]
    pub unable_oci_other_yoy: Option<f64>,
    ///
    #[serde(rename = "UNABLE_OCI_BALANCE")]
    pub unable_oci_balance: Option<f64>,
    ///
    #[serde(rename = "UNABLE_OCI_BALANCE_YOY")]
    pub unable_oci_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "ABLE_OCI")]
    pub able_oci: Option<f64>,
    ///
    #[serde(rename = "ABLE_OCI_YOY")]
    pub able_oci_yoy: Option<f64>,
    ///
    #[serde(rename = "RIGHTLAW_ABLE_OCI")]
    pub rightlaw_able_oci: Option<f64>,
    ///
    #[serde(rename = "RIGHTLAW_ABLE_OCI_YOY")]
    pub rightlaw_able_oci_yoy: Option<f64>,
    ///
    #[serde(rename = "AFA_FAIRVALUE_CHANGE")]
    pub afa_fairvalue_change: Option<f64>,
    ///
    #[serde(rename = "AFA_FAIRVALUE_CHANGE_YOY")]
    pub afa_fairvalue_change_yoy: Option<f64>,
    ///
    #[serde(rename = "HMI_AFA")]
    pub hmi_afa: Option<f64>,
    ///
    #[serde(rename = "HMI_AFA_YOY")]
    pub hmi_afa_yoy: Option<f64>,
    ///
    #[serde(rename = "CASHFLOW_HEDGE_VALID")]
    pub cashflow_hedge_valid: Option<f64>,
    ///
    #[serde(rename = "CASHFLOW_HEDGE_VALID_YOY")]
    pub cashflow_hedge_valid_yoy: Option<f64>,
    ///
    #[serde(rename = "CREDITOR_FAIRVALUE_CHANGE")]
    pub creditor_fairvalue_change: Option<f64>,
    ///
    #[serde(rename = "CREDITOR_FAIRVALUE_CHANGE_YOY")]
    pub creditor_fairvalue_change_yoy: Option<f64>,
    ///
    #[serde(rename = "CREDITOR_IMPAIRMENT_RESERVE")]
    pub creditor_impairment_reserve: Option<f64>,
    ///
    #[serde(rename = "CREDITOR_IMPAIRMENT_RESERVE_YOY")]
    pub creditor_impairment_reserve_yoy: Option<f64>,
    ///
    #[serde(rename = "FINANCE_OCI_AMT")]
    pub finance_oci_amt: Option<f64>,
    ///
    #[serde(rename = "FINANCE_OCI_AMT_YOY")]
    pub finance_oci_amt_yoy: Option<f64>,
    ///
    #[serde(rename = "CONVERT_DIFF")]
    pub convert_diff: Option<f64>,
    ///
    #[serde(rename = "CONVERT_DIFF_YOY")]
    pub convert_diff_yoy: Option<f64>,
    ///
    #[serde(rename = "ABLE_OCI_OTHER")]
    pub able_oci_other: Option<f64>,
    ///
    #[serde(rename = "ABLE_OCI_OTHER_YOY")]
    pub able_oci_other_yoy: Option<f64>,
    ///
    #[serde(rename = "ABLE_OCI_BALANCE")]
    pub able_oci_balance: Option<f64>,
    ///
    #[serde(rename = "ABLE_OCI_BALANCE_YOY")]
    pub able_oci_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "OCI_OTHER")]
    pub oci_other: Option<f64>,
    ///
    #[serde(rename = "OCI_OTHER_YOY")]
    pub oci_other_yoy: Option<f64>,
    ///
    #[serde(rename = "OCI_BALANCE")]
    pub oci_balance: Option<f64>,
    ///
    #[serde(rename = "OCI_BALANCE_YOY")]
    pub oci_balance_yoy: Option<f64>,
    /// 综合收益总额
    #[serde(rename = "TOTAL_COMPRE_INCOME")]
    pub total_compre_income: Option<f64>,
    ///
    #[serde(rename = "TOTAL_COMPRE_INCOME_YOY")]
    pub total_compre_income_yoy: Option<f64>,
    /// 综合收益总额  归属于母公司股东的综合收益总额
    #[serde(rename = "PARENT_TCI")]
    pub parent_tci: Option<f64>,
    ///
    #[serde(rename = "PARENT_TCI_YOY")]
    pub parent_tci_yoy: Option<f64>,
    /// 综合收益总额  归属于少数股东的综合收益总额
    #[serde(rename = "MINORITY_TCI")]
    pub minority_tci: Option<f64>,
    ///
    #[serde(rename = "MINORITY_TCI_YOY")]
    pub minority_tci_yoy: Option<f64>,
    ///
    #[serde(rename = "PRECOMBINE_TCI")]
    pub precombine_tci: Option<f64>,
    ///
    #[serde(rename = "PRECOMBINE_TCI_YOY")]
    pub precombine_tci_yoy: Option<f64>,
    ///
    #[serde(rename = "EFFECT_TCI_BALANCE")]
    pub effect_tci_balance: Option<f64>,
    ///
    #[serde(rename = "EFFECT_TCI_BALANCE_YOY")]
    pub effect_tci_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "TCI_OTHER")]
    pub tci_other: Option<f64>,
    ///
    #[serde(rename = "TCI_OTHER_YOY")]
    pub tci_other_yoy: Option<f64>,
    ///
    #[serde(rename = "TCI_BALANCE")]
    pub tci_balance: Option<f64>,
    ///
    #[serde(rename = "TCI_BALANCE_YOY")]
    pub tci_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "ACF_END_INCOME")]
    pub acf_end_income: Option<f64>,
    ///
    #[serde(rename = "ACF_END_INCOME_YOY")]
    pub acf_end_income_yoy: Option<f64>,
    /// 审计意见
    #[serde(rename = "OPINION_TYPE")]
    pub opinion_type: Option<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let akshare = Akshare::new();
        if let Ok(client) = akshare {
            let s = client.get_income_statement(&"SZ002027".into()).await.unwrap();
            print!("{:?}", s.get("2021-12-31 00:00:00"));
        }
    }
}
