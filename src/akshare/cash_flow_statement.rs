use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::Result;

use super::{Akshare, StockCode};

impl Akshare {
    pub async fn get_cash_flow_statement(
        &self,
        symbol: &StockCode,
    ) -> Result<HashMap<String, CashFlowStatementData>> {
        let data: Vec<CashFlowStatementData> = self
            .transport
            .get("stock_cash_flow_sheet_by_report_em", Some(symbol))
            .await?;
        let mut map = HashMap::with_capacity(data.len());
        for cash_data in data {
            map.insert(cash_data.report_date.to_owned(), cash_data);
        }
        Ok(map)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CashFlowStatementData {
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
    /// 经营活动产生的现金流量  销售商品、提供劳务收到的现金
    #[serde(rename = "SALES_SERVICES")]
    pub sales_services: Option<f64>,
    ///
    #[serde(rename = "DEPOSIT_INTERBANK_ADD")]
    pub deposit_interbank_add: Option<f64>,
    ///
    #[serde(rename = "LOAN_PBC_ADD")]
    pub loan_pbc_add: Option<f64>,
    ///
    #[serde(rename = "OFI_BF_ADD")]
    pub ofi_bf_add: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_ORIGIC_PREMIUM")]
    pub receive_origic_premium: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_REINSURE_NET")]
    pub receive_reinsure_net: Option<f64>,
    ///
    #[serde(rename = "INSURED_INVEST_ADD")]
    pub insured_invest_add: Option<f64>,
    ///
    #[serde(rename = "DISPOSAL_TFA_ADD")]
    pub disposal_tfa_add: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_INTEREST_COMMISSION")]
    pub receive_interest_commission: Option<f64>,
    ///
    #[serde(rename = "BORROW_FUND_ADD")]
    pub borrow_fund_add: Option<f64>,
    ///
    #[serde(rename = "LOAN_ADVANCE_REDUCE")]
    pub loan_advance_reduce: Option<f64>,
    ///
    #[serde(rename = "REPO_BUSINESS_ADD")]
    pub repo_business_add: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_TAX_REFUND")]
    pub receive_tax_refund: Option<f64>,
    /// 经营活动产生的现金流量  收到其他与经营活动有关的现金
    #[serde(rename = "RECEIVE_OTHER_OPERATE")]
    pub receive_other_operate: Option<f64>,
    ///
    #[serde(rename = "OPERATE_INFLOW_OTHER")]
    pub operate_inflow_other: Option<f64>,
    ///
    #[serde(rename = "OPERATE_INFLOW_BALANCE")]
    pub operate_inflow_balance: Option<f64>,
    /// 经营活动现金流入小计
    #[serde(rename = "TOTAL_OPERATE_INFLOW")]
    pub total_operate_inflow: Option<f64>,
    /// 经营活动现金流入小计  购买商品、接受劳务支付的现金
    #[serde(rename = "BUY_SERVICES")]
    pub buy_services: Option<f64>,
    ///
    #[serde(rename = "LOAN_ADVANCE_ADD")]
    pub loan_advance_add: Option<f64>,
    ///
    #[serde(rename = "PBC_INTERBANK_ADD")]
    pub pbc_interbank_add: Option<f64>,
    ///
    #[serde(rename = "PAY_ORIGIC_COMPENSATE")]
    pub pay_origic_compensate: Option<f64>,
    ///
    #[serde(rename = "PAY_INTEREST_COMMISSION")]
    pub pay_interest_commission: Option<f64>,
    ///
    #[serde(rename = "PAY_POLICY_BONUS")]
    pub pay_policy_bonus: Option<f64>,
    /// 经营活动现金流入小计  支付给职工以及为职工支付的现金
    #[serde(rename = "PAY_STAFF_CASH")]
    pub pay_staff_cash: Option<f64>,
    /// 经营活动现金流入小计  支付的各项税费
    #[serde(rename = "PAY_ALL_TAX")]
    pub pay_all_tax: Option<f64>,
    /// 经营活动现金流入小计  支付其他与经营活动有关的现金
    #[serde(rename = "PAY_OTHER_OPERATE")]
    pub pay_other_operate: Option<f64>,
    ///
    #[serde(rename = "OPERATE_OUTFLOW_OTHER")]
    pub operate_outflow_other: Option<f64>,
    ///
    #[serde(rename = "OPERATE_OUTFLOW_BALANCE")]
    pub operate_outflow_balance: Option<f64>,
    /// 经营活动现金流出小计
    #[serde(rename = "TOTAL_OPERATE_OUTFLOW")]
    pub total_operate_outflow: Option<f64>,
    ///
    #[serde(rename = "OPERATE_NETCASH_OTHER")]
    pub operate_netcash_other: Option<f64>,
    ///
    #[serde(rename = "OPERATE_NETCASH_BALANCE")]
    pub operate_netcash_balance: Option<f64>,
    /// 经营活动产生的现金流量净额
    #[serde(rename = "NETCASH_OPERATE")]
    pub netcash_operate: Option<f64>,
    /// 投资活动产生的现金流量  收回投资收到的现金
    #[serde(rename = "WITHDRAW_INVEST")]
    pub withdraw_invest: Option<f64>,
    /// 投资活动产生的现金流量  取得投资收益收到的现金
    #[serde(rename = "RECEIVE_INVEST_INCOME")]
    pub receive_invest_income: Option<f64>,
    /// 投资活动产生的现金流量  处置固定资产、无形资产和其他长期资产收回的现金净额
    #[serde(rename = "DISPOSAL_LONG_ASSET")]
    pub disposal_long_asset: Option<f64>,
    /// 投资活动产生的现金流量  处置子公司及其他营业单位收到的现金
    #[serde(rename = "DISPOSAL_SUBSIDIARY_OTHER")]
    pub disposal_subsidiary_other: Option<f64>,
    ///
    #[serde(rename = "REDUCE_PLEDGE_TIMEDEPOSITS")]
    pub reduce_pledge_timedeposits: Option<f64>,
    /// 投资活动产生的现金流量  收到的其他与投资活动有关的现金
    #[serde(rename = "RECEIVE_OTHER_INVEST")]
    pub receive_other_invest: Option<f64>,
    ///
    #[serde(rename = "INVEST_INFLOW_OTHER")]
    pub invest_inflow_other: Option<f64>,
    ///
    #[serde(rename = "INVEST_INFLOW_BALANCE")]
    pub invest_inflow_balance: Option<f64>,
    /// 投资活动现金流入小计
    #[serde(rename = "TOTAL_INVEST_INFLOW")]
    pub total_invest_inflow: Option<f64>,
    /// 投资活动现金流入小计  购建固定资产、无形资产和其他长期资产支付的现金
    #[serde(rename = "CONSTRUCT_LONG_ASSET")]
    pub construct_long_asset: Option<f64>,
    /// 投资活动现金流入小计  投资支付的现金
    #[serde(rename = "INVEST_PAY_CASH")]
    pub invest_pay_cash: Option<f64>,
    ///
    #[serde(rename = "PLEDGE_LOAN_ADD")]
    pub pledge_loan_add: Option<f64>,
    ///
    #[serde(rename = "OBTAIN_SUBSIDIARY_OTHER")]
    pub obtain_subsidiary_other: Option<f64>,
    ///
    #[serde(rename = "ADD_PLEDGE_TIMEDEPOSITS")]
    pub add_pledge_timedeposits: Option<f64>,
    /// 投资活动现金流入小计 支付其他与投资活动有关的现金
    #[serde(rename = "PAY_OTHER_INVEST")]
    pub pay_other_invest: Option<f64>,
    ///
    #[serde(rename = "INVEST_OUTFLOW_OTHER")]
    pub invest_outflow_other: Option<f64>,
    ///
    #[serde(rename = "INVEST_OUTFLOW_BALANCE")]
    pub invest_outflow_balance: Option<f64>,
    /// 投资活动现金流出小计
    #[serde(rename = "TOTAL_INVEST_OUTFLOW")]
    pub total_invest_outflow: Option<f64>,
    ///
    #[serde(rename = "INVEST_NETCASH_OTHER")]
    pub invest_netcash_other: Option<f64>,
    ///
    #[serde(rename = "INVEST_NETCASH_BALANCE")]
    pub invest_netcash_balance: Option<f64>,
    /// 投资活动产生的现金流量净额
    #[serde(rename = "NETCASH_INVEST")]
    pub netcash_invest: Option<f64>,
    /// 筹资活动产生的现金流量  吸收投资收到的现金
    #[serde(rename = "ACCEPT_INVEST_CASH")]
    pub accept_invest_cash: Option<f64>,
    /// 筹资活动产生的现金流量  其中:子公司吸收少数股东投资收到的现金
    #[serde(rename = "SUBSIDIARY_ACCEPT_INVEST")]
    pub subsidiary_accept_invest: Option<f64>,
    /// 取得借款收到的现金
    #[serde(rename = "RECEIVE_LOAN_CASH")]
    pub receive_loan_cash: Option<f64>,
    /// 发行债券
    #[serde(rename = "ISSUE_BOND")]
    pub issue_bond: Option<f64>,
    /// 收到的其他与筹资活动有关的现金
    #[serde(rename = "RECEIVE_OTHER_FINANCE")]
    pub receive_other_finance: Option<f64>,
    ///
    #[serde(rename = "FINANCE_INFLOW_OTHER")]
    pub finance_inflow_other: Option<f64>,
    ///
    #[serde(rename = "FINANCE_INFLOW_BALANCE")]
    pub finance_inflow_balance: Option<f64>,
    /// 筹资活动现金流入小计
    #[serde(rename = "TOTAL_FINANCE_INFLOW")]
    pub total_finance_inflow: Option<f64>,
    /// 筹资活动现金流出小计  偿还债务所支付的现金
    #[serde(rename = "PAY_DEBT_CASH")]
    pub pay_debt_cash: Option<f64>,
    /// 筹资活动现金流出小计  分配股利、利润或偿付利息支付的现金
    #[serde(rename = "ASSIGN_DIVIDEND_PORFIT")]
    pub assign_dividend_porfit: Option<f64>,
    /// 筹资活动现金流出小计  其中:子公司支付给少数股东的股利、利润
    #[serde(rename = "SUBSIDIARY_PAY_DIVIDEND")]
    pub subsidiary_pay_dividend: Option<f64>,
    ///
    #[serde(rename = "BUY_SUBSIDIARY_EQUITY")]
    pub buy_subsidiary_equity: Option<f64>,
    /// 筹资活动现金流出小计  支付的其他与筹资活动有关的现金
    #[serde(rename = "PAY_OTHER_FINANCE")]
    pub pay_other_finance: Option<f64>,
    ///
    #[serde(rename = "SUBSIDIARY_REDUCE_CASH")]
    pub subsidiary_reduce_cash: Option<f64>,
    ///
    #[serde(rename = "FINANCE_OUTFLOW_OTHER")]
    pub finance_outflow_other: Option<f64>,
    ///
    #[serde(rename = "FINANCE_OUTFLOW_BALANCE")]
    pub finance_outflow_balance: Option<f64>,
    /// 筹资活动现金流出小计
    #[serde(rename = "TOTAL_FINANCE_OUTFLOW")]
    pub total_finance_outflow: Option<f64>,
    ///
    #[serde(rename = "FINANCE_NETCASH_OTHER")]
    pub finance_netcash_other: Option<f64>,
    ///
    #[serde(rename = "FINANCE_NETCASH_BALANCE")]
    pub finance_netcash_balance: Option<f64>,
    /// 筹资活动产生的现金流量净额
    #[serde(rename = "NETCASH_FINANCE")]
    pub netcash_finance: Option<f64>,
    /// 汇率变动对现金及现金等价物的影响
    #[serde(rename = "RATE_CHANGE_EFFECT")]
    pub rate_change_effect: Option<f64>,
    ///
    #[serde(rename = "CCE_ADD_OTHER")]
    pub cce_add_other: Option<f64>,
    ///
    #[serde(rename = "CCE_ADD_BALANCE")]
    pub cce_add_balance: Option<f64>,
    /// 现金及现金等价物净增加额
    #[serde(rename = "CCE_ADD")]
    pub cce_add: Option<f64>,
    /// 加:期初现金及现金等价物余额
    #[serde(rename = "BEGIN_CCE")]
    pub begin_cce: Option<f64>,
    ///
    #[serde(rename = "END_CCE_OTHER")]
    pub end_cce_other: Option<f64>,
    ///
    #[serde(rename = "END_CCE_BALANCE")]
    pub end_cce_balance: Option<f64>,
    /// 期末现金及现金等价物余额
    #[serde(rename = "END_CCE")]
    pub end_cce: Option<f64>,
    /// 净利润
    #[serde(rename = "NETPROFIT")]
    pub netprofit: Option<f64>,
    /// 资产减值准备
    #[serde(rename = "ASSET_IMPAIRMENT")]
    pub asset_impairment: Option<f64>,
    /// 固定资产和投资性房地产折旧
    #[serde(rename = "FA_IR_DEPR")]
    pub fa_ir_depr: Option<f64>,
    /// 其中:固定资产折旧、油气资产折耗、生产性生物资产折旧
    #[serde(rename = "OILGAS_BIOLOGY_DEPR")]
    pub oilgas_biology_depr: Option<f64>,
    ///
    #[serde(rename = "IR_DEPR")]
    pub ir_depr: Option<f64>,
    /// 无形资产摊销
    #[serde(rename = "IA_AMORTIZE")]
    pub ia_amortize: Option<f64>,
    /// 长期待摊费用摊销
    #[serde(rename = "LPE_AMORTIZE")]
    pub lpe_amortize: Option<f64>,
    ///
    #[serde(rename = "DEFER_INCOME_AMORTIZE")]
    pub defer_income_amortize: Option<f64>,
    ///
    #[serde(rename = "PREPAID_EXPENSE_REDUCE")]
    pub prepaid_expense_reduce: Option<f64>,
    ///
    #[serde(rename = "ACCRUED_EXPENSE_ADD")]
    pub accrued_expense_add: Option<f64>,
    /// 处置固定资产、无形资产和其他长期资产的损失
    #[serde(rename = "DISPOSAL_LONGASSET_LOSS")]
    pub disposal_longasset_loss: Option<f64>,
    /// 固定资产报废损失
    #[serde(rename = "FA_SCRAP_LOSS")]
    pub fa_scrap_loss: Option<f64>,
    /// 公允价值变动损失
    #[serde(rename = "FAIRVALUE_CHANGE_LOSS")]
    pub fairvalue_change_loss: Option<f64>,
    /// 财务费用
    #[serde(rename = "FINANCE_EXPENSE")]
    pub finance_expense: Option<f64>,
    /// 投资损失
    #[serde(rename = "INVEST_LOSS")]
    pub invest_loss: Option<f64>,
    /// 递延所得税
    #[serde(rename = "DEFER_TAX")]
    pub defer_tax: Option<f64>,
    /// 其中:递延所得税资产减少
    #[serde(rename = "DT_ASSET_REDUCE")]
    pub dt_asset_reduce: Option<f64>,
    /// 递延所得税负债增加
    #[serde(rename = "DT_LIAB_ADD")]
    pub dt_liab_add: Option<f64>,
    ///
    #[serde(rename = "PREDICT_LIAB_ADD")]
    pub predict_liab_add: Option<f64>,
    /// 存货的减少
    #[serde(rename = "INVENTORY_REDUCE")]
    pub inventory_reduce: Option<f64>,
    /// 经营性应收项目的减少
    #[serde(rename = "OPERATE_RECE_REDUCE")]
    pub operate_rece_reduce: Option<f64>,
    /// 经营性应付项目的增加
    #[serde(rename = "OPERATE_PAYABLE_ADD")]
    pub operate_payable_add: Option<f64>,
    ///
    #[serde(rename = "OTHER")]
    pub other: Option<f64>,
    ///
    #[serde(rename = "OPERATE_NETCASH_OTHERNOTE")]
    pub operate_netcash_othernote: Option<f64>,
    ///
    #[serde(rename = "OPERATE_NETCASH_BALANCENOTE")]
    pub operate_netcash_balancenote: Option<f64>,
    ///
    #[serde(rename = "NETCASH_OPERATENOTE")]
    pub netcash_operatenote: Option<f64>,
    ///
    #[serde(rename = "DEBT_TRANSFER_CAPITAL")]
    pub debt_transfer_capital: Option<f64>,
    ///
    #[serde(rename = "CONVERT_BOND_1YEAR")]
    pub convert_bond_1year: Option<f64>,
    ///
    #[serde(rename = "FINLEASE_OBTAIN_FA")]
    pub finlease_obtain_fa: Option<f64>,
    ///
    #[serde(rename = "UNINVOLVE_INVESTFIN_OTHER")]
    pub uninvolve_investfin_other: Option<f64>,
    /// 现金的期末余额
    #[serde(rename = "END_CASH")]
    pub end_cash: Option<f64>,
    /// 减:现金的期初余额
    #[serde(rename = "BEGIN_CASH")]
    pub begin_cash: Option<f64>,
    ///
    #[serde(rename = "END_CASH_EQUIVALENTS")]
    pub end_cash_equivalents: Option<f64>,
    ///
    #[serde(rename = "BEGIN_CASH_EQUIVALENTS")]
    pub begin_cash_equivalents: Option<f64>,
    ///
    #[serde(rename = "CCE_ADD_OTHERNOTE")]
    pub cce_add_othernote: Option<f64>,
    ///
    #[serde(rename = "CCE_ADD_BALANCENOTE")]
    pub cce_add_balancenote: Option<f64>,
    ///
    #[serde(rename = "CCE_ADDNOTE")]
    pub cce_addnote: Option<f64>,
    ///
    #[serde(rename = "SALES_SERVICES_YOY")]
    pub sales_services_yoy: Option<f64>,
    ///
    #[serde(rename = "DEPOSIT_INTERBANK_ADD_YOY")]
    pub deposit_interbank_add_yoy: Option<f64>,
    ///
    #[serde(rename = "LOAN_PBC_ADD_YOY")]
    pub loan_pbc_add_yoy: Option<f64>,
    ///
    #[serde(rename = "OFI_BF_ADD_YOY")]
    pub ofi_bf_add_yoy: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_ORIGIC_PREMIUM_YOY")]
    pub receive_origic_premium_yoy: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_REINSURE_NET_YOY")]
    pub receive_reinsure_net_yoy: Option<f64>,
    ///
    #[serde(rename = "INSURED_INVEST_ADD_YOY")]
    pub insured_invest_add_yoy: Option<f64>,
    ///
    #[serde(rename = "DISPOSAL_TFA_ADD_YOY")]
    pub disposal_tfa_add_yoy: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_INTEREST_COMMISSION_YOY")]
    pub receive_interest_commission_yoy: Option<f64>,
    ///
    #[serde(rename = "BORROW_FUND_ADD_YOY")]
    pub borrow_fund_add_yoy: Option<f64>,
    ///
    #[serde(rename = "LOAN_ADVANCE_REDUCE_YOY")]
    pub loan_advance_reduce_yoy: Option<f64>,
    ///
    #[serde(rename = "REPO_BUSINESS_ADD_YOY")]
    pub repo_business_add_yoy: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_TAX_REFUND_YOY")]
    pub receive_tax_refund_yoy: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_OTHER_OPERATE_YOY")]
    pub receive_other_operate_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_INFLOW_OTHER_YOY")]
    pub operate_inflow_other_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_INFLOW_BALANCE_YOY")]
    pub operate_inflow_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_OPERATE_INFLOW_YOY")]
    pub total_operate_inflow_yoy: Option<f64>,
    ///
    #[serde(rename = "BUY_SERVICES_YOY")]
    pub buy_services_yoy: Option<f64>,
    ///
    #[serde(rename = "LOAN_ADVANCE_ADD_YOY")]
    pub loan_advance_add_yoy: Option<f64>,
    ///
    #[serde(rename = "PBC_INTERBANK_ADD_YOY")]
    pub pbc_interbank_add_yoy: Option<f64>,
    ///
    #[serde(rename = "PAY_ORIGIC_COMPENSATE_YOY")]
    pub pay_origic_compensate_yoy: Option<f64>,
    ///
    #[serde(rename = "PAY_INTEREST_COMMISSION_YOY")]
    pub pay_interest_commission_yoy: Option<f64>,
    ///
    #[serde(rename = "PAY_POLICY_BONUS_YOY")]
    pub pay_policy_bonus_yoy: Option<f64>,
    ///
    #[serde(rename = "PAY_STAFF_CASH_YOY")]
    pub pay_staff_cash_yoy: Option<f64>,
    ///
    #[serde(rename = "PAY_ALL_TAX_YOY")]
    pub pay_all_tax_yoy: Option<f64>,
    ///
    #[serde(rename = "PAY_OTHER_OPERATE_YOY")]
    pub pay_other_operate_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_OUTFLOW_OTHER_YOY")]
    pub operate_outflow_other_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_OUTFLOW_BALANCE_YOY")]
    pub operate_outflow_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_OPERATE_OUTFLOW_YOY")]
    pub total_operate_outflow_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_NETCASH_OTHER_YOY")]
    pub operate_netcash_other_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_NETCASH_BALANCE_YOY")]
    pub operate_netcash_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "NETCASH_OPERATE_YOY")]
    pub netcash_operate_yoy: Option<f64>,
    ///
    #[serde(rename = "WITHDRAW_INVEST_YOY")]
    pub withdraw_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_INVEST_INCOME_YOY")]
    pub receive_invest_income_yoy: Option<f64>,
    ///
    #[serde(rename = "DISPOSAL_LONG_ASSET_YOY")]
    pub disposal_long_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "DISPOSAL_SUBSIDIARY_OTHER_YOY")]
    pub disposal_subsidiary_other_yoy: Option<f64>,
    ///
    #[serde(rename = "REDUCE_PLEDGE_TIMEDEPOSITS_YOY")]
    pub reduce_pledge_timedeposits_yoy: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_OTHER_INVEST_YOY")]
    pub receive_other_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "INVEST_INFLOW_OTHER_YOY")]
    pub invest_inflow_other_yoy: Option<f64>,
    ///
    #[serde(rename = "INVEST_INFLOW_BALANCE_YOY")]
    pub invest_inflow_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_INVEST_INFLOW_YOY")]
    pub total_invest_inflow_yoy: Option<f64>,
    ///
    #[serde(rename = "CONSTRUCT_LONG_ASSET_YOY")]
    pub construct_long_asset_yoy: Option<f64>,
    ///
    #[serde(rename = "INVEST_PAY_CASH_YOY")]
    pub invest_pay_cash_yoy: Option<f64>,
    ///
    #[serde(rename = "PLEDGE_LOAN_ADD_YOY")]
    pub pledge_loan_add_yoy: Option<f64>,
    ///
    #[serde(rename = "OBTAIN_SUBSIDIARY_OTHER_YOY")]
    pub obtain_subsidiary_other_yoy: Option<f64>,
    ///
    #[serde(rename = "ADD_PLEDGE_TIMEDEPOSITS_YOY")]
    pub add_pledge_timedeposits_yoy: Option<f64>,
    ///
    #[serde(rename = "PAY_OTHER_INVEST_YOY")]
    pub pay_other_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "INVEST_OUTFLOW_OTHER_YOY")]
    pub invest_outflow_other_yoy: Option<f64>,
    ///
    #[serde(rename = "INVEST_OUTFLOW_BALANCE_YOY")]
    pub invest_outflow_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_INVEST_OUTFLOW_YOY")]
    pub total_invest_outflow_yoy: Option<f64>,
    ///
    #[serde(rename = "INVEST_NETCASH_OTHER_YOY")]
    pub invest_netcash_other_yoy: Option<f64>,
    ///
    #[serde(rename = "INVEST_NETCASH_BALANCE_YOY")]
    pub invest_netcash_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "NETCASH_INVEST_YOY")]
    pub netcash_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "ACCEPT_INVEST_CASH_YOY")]
    pub accept_invest_cash_yoy: Option<f64>,
    ///
    #[serde(rename = "SUBSIDIARY_ACCEPT_INVEST_YOY")]
    pub subsidiary_accept_invest_yoy: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_LOAN_CASH_YOY")]
    pub receive_loan_cash_yoy: Option<f64>,
    ///
    #[serde(rename = "ISSUE_BOND_YOY")]
    pub issue_bond_yoy: Option<f64>,
    ///
    #[serde(rename = "RECEIVE_OTHER_FINANCE_YOY")]
    pub receive_other_finance_yoy: Option<f64>,
    ///
    #[serde(rename = "FINANCE_INFLOW_OTHER_YOY")]
    pub finance_inflow_other_yoy: Option<f64>,
    ///
    #[serde(rename = "FINANCE_INFLOW_BALANCE_YOY")]
    pub finance_inflow_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_FINANCE_INFLOW_YOY")]
    pub total_finance_inflow_yoy: Option<f64>,
    ///
    #[serde(rename = "PAY_DEBT_CASH_YOY")]
    pub pay_debt_cash_yoy: Option<f64>,
    ///
    #[serde(rename = "ASSIGN_DIVIDEND_PORFIT_YOY")]
    pub assign_dividend_porfit_yoy: Option<f64>,
    ///
    #[serde(rename = "SUBSIDIARY_PAY_DIVIDEND_YOY")]
    pub subsidiary_pay_dividend_yoy: Option<f64>,
    ///
    #[serde(rename = "BUY_SUBSIDIARY_EQUITY_YOY")]
    pub buy_subsidiary_equity_yoy: Option<f64>,
    ///
    #[serde(rename = "PAY_OTHER_FINANCE_YOY")]
    pub pay_other_finance_yoy: Option<f64>,
    ///
    #[serde(rename = "SUBSIDIARY_REDUCE_CASH_YOY")]
    pub subsidiary_reduce_cash_yoy: Option<f64>,
    ///
    #[serde(rename = "FINANCE_OUTFLOW_OTHER_YOY")]
    pub finance_outflow_other_yoy: Option<f64>,
    ///
    #[serde(rename = "FINANCE_OUTFLOW_BALANCE_YOY")]
    pub finance_outflow_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "TOTAL_FINANCE_OUTFLOW_YOY")]
    pub total_finance_outflow_yoy: Option<f64>,
    ///
    #[serde(rename = "FINANCE_NETCASH_OTHER_YOY")]
    pub finance_netcash_other_yoy: Option<f64>,
    ///
    #[serde(rename = "FINANCE_NETCASH_BALANCE_YOY")]
    pub finance_netcash_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "NETCASH_FINANCE_YOY")]
    pub netcash_finance_yoy: Option<f64>,
    ///
    #[serde(rename = "RATE_CHANGE_EFFECT_YOY")]
    pub rate_change_effect_yoy: Option<f64>,
    ///
    #[serde(rename = "CCE_ADD_OTHER_YOY")]
    pub cce_add_other_yoy: Option<f64>,
    ///
    #[serde(rename = "CCE_ADD_BALANCE_YOY")]
    pub cce_add_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "CCE_ADD_YOY")]
    pub cce_add_yoy: Option<f64>,
    ///
    #[serde(rename = "BEGIN_CCE_YOY")]
    pub begin_cce_yoy: Option<f64>,
    ///
    #[serde(rename = "END_CCE_OTHER_YOY")]
    pub end_cce_other_yoy: Option<f64>,
    ///
    #[serde(rename = "END_CCE_BALANCE_YOY")]
    pub end_cce_balance_yoy: Option<f64>,
    ///
    #[serde(rename = "END_CCE_YOY")]
    pub end_cce_yoy: Option<f64>,
    ///
    #[serde(rename = "NETPROFIT_YOY")]
    pub netprofit_yoy: Option<f64>,
    ///
    #[serde(rename = "ASSET_IMPAIRMENT_YOY")]
    pub asset_impairment_yoy: Option<f64>,
    ///
    #[serde(rename = "FA_IR_DEPR_YOY")]
    pub fa_ir_depr_yoy: Option<f64>,
    ///
    #[serde(rename = "OILGAS_BIOLOGY_DEPR_YOY")]
    pub oilgas_biology_depr_yoy: Option<f64>,
    ///
    #[serde(rename = "IR_DEPR_YOY")]
    pub ir_depr_yoy: Option<f64>,
    ///
    #[serde(rename = "IA_AMORTIZE_YOY")]
    pub ia_amortize_yoy: Option<f64>,
    ///
    #[serde(rename = "LPE_AMORTIZE_YOY")]
    pub lpe_amortize_yoy: Option<f64>,
    ///
    #[serde(rename = "DEFER_INCOME_AMORTIZE_YOY")]
    pub defer_income_amortize_yoy: Option<f64>,
    ///
    #[serde(rename = "PREPAID_EXPENSE_REDUCE_YOY")]
    pub prepaid_expense_reduce_yoy: Option<f64>,
    ///
    #[serde(rename = "ACCRUED_EXPENSE_ADD_YOY")]
    pub accrued_expense_add_yoy: Option<f64>,
    ///
    #[serde(rename = "DISPOSAL_LONGASSET_LOSS_YOY")]
    pub disposal_longasset_loss_yoy: Option<f64>,
    ///
    #[serde(rename = "FA_SCRAP_LOSS_YOY")]
    pub fa_scrap_loss_yoy: Option<f64>,
    ///
    #[serde(rename = "FAIRVALUE_CHANGE_LOSS_YOY")]
    pub fairvalue_change_loss_yoy: Option<f64>,
    ///
    #[serde(rename = "FINANCE_EXPENSE_YOY")]
    pub finance_expense_yoy: Option<f64>,
    ///
    #[serde(rename = "INVEST_LOSS_YOY")]
    pub invest_loss_yoy: Option<f64>,
    ///
    #[serde(rename = "DEFER_TAX_YOY")]
    pub defer_tax_yoy: Option<f64>,
    ///
    #[serde(rename = "DT_ASSET_REDUCE_YOY")]
    pub dt_asset_reduce_yoy: Option<f64>,
    ///
    #[serde(rename = "DT_LIAB_ADD_YOY")]
    pub dt_liab_add_yoy: Option<f64>,
    ///
    #[serde(rename = "PREDICT_LIAB_ADD_YOY")]
    pub predict_liab_add_yoy: Option<f64>,
    ///
    #[serde(rename = "INVENTORY_REDUCE_YOY")]
    pub inventory_reduce_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_RECE_REDUCE_YOY")]
    pub operate_rece_reduce_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_PAYABLE_ADD_YOY")]
    pub operate_payable_add_yoy: Option<f64>,
    ///
    #[serde(rename = "OTHER_YOY")]
    pub other_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_NETCASH_OTHERNOTE_YOY")]
    pub operate_netcash_othernote_yoy: Option<f64>,
    ///
    #[serde(rename = "OPERATE_NETCASH_BALANCENOTE_YOY")]
    pub operate_netcash_balancenote_yoy: Option<f64>,
    ///
    #[serde(rename = "NETCASH_OPERATENOTE_YOY")]
    pub netcash_operatenote_yoy: Option<f64>,
    ///
    #[serde(rename = "DEBT_TRANSFER_CAPITAL_YOY")]
    pub debt_transfer_capital_yoy: Option<f64>,
    ///
    #[serde(rename = "CONVERT_BOND_1YEAR_YOY")]
    pub convert_bond_1year_yoy: Option<f64>,
    ///
    #[serde(rename = "FINLEASE_OBTAIN_FA_YOY")]
    pub finlease_obtain_fa_yoy: Option<f64>,
    ///
    #[serde(rename = "UNINVOLVE_INVESTFIN_OTHER_YOY")]
    pub uninvolve_investfin_other_yoy: Option<f64>,
    ///
    #[serde(rename = "END_CASH_YOY")]
    pub end_cash_yoy: Option<f64>,
    ///
    #[serde(rename = "BEGIN_CASH_YOY")]
    pub begin_cash_yoy: Option<f64>,
    ///
    #[serde(rename = "END_CASH_EQUIVALENTS_YOY")]
    pub end_cash_equivalents_yoy: Option<f64>,
    ///
    #[serde(rename = "BEGIN_CASH_EQUIVALENTS_YOY")]
    pub begin_cash_equivalents_yoy: Option<f64>,
    ///
    #[serde(rename = "CCE_ADD_OTHERNOTE_YOY")]
    pub cce_add_othernote_yoy: Option<f64>,
    ///
    #[serde(rename = "CCE_ADD_BALANCENOTE_YOY")]
    pub cce_add_balancenote_yoy: Option<f64>,
    ///
    #[serde(rename = "CCE_ADDNOTE_YOY")]
    pub cce_addnote_yoy: Option<f64>,
    ///  审计意见
    #[serde(rename = "OPINION_TYPE")]
    pub opinion_type: Option<String>,
    ///
    #[serde(rename = "OSOPINION_TYPE")]
    pub osopinion_type: Option<String>,
    ///
    #[serde(rename = "MINORITY_INTEREST")]
    pub minority_interest: Option<f64>,
    ///
    #[serde(rename = "MINORITY_INTEREST_YOY")]
    pub minority_interest_yoy: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let akshare = Akshare::new();
        if let Ok(client) = akshare {
            let s = client.get_cash_flow_statement(&"SZ002027".into()).await.unwrap();
            print!("{:?}", s.get("2021-12-31 00:00:00"));
        }
    }
}
