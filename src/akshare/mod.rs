pub mod balance_sheet;
pub mod cash_flow_statement;
pub mod income_statement;
pub mod model;
pub mod shared;
mod transport;

use std::fmt;

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::errors::Result;

use self::transport::Transport;

#[derive(Clone)]
pub struct Akshare {
    pub transport: Transport,
}

impl Akshare {
    pub fn new() -> Result<Self> {
        let t = Transport::new()?;
        Ok(Self { transport: t })
    }

    pub fn from(url: &str) -> Result<Self> {
        let t = Transport::from(url)?;
        Ok(Self { transport: t })
    }

    pub async fn sh_list(&self) -> Result<Vec<StockInfo>> {
        self.code_list("stock_info_sh_name_code").await
    }

    pub async fn sz_list(&self) -> Result<Vec<StockInfo>> {
        self.code_list("stock_info_sz_name_code").await
    }

    pub async fn code_list(&self, point: &str) -> Result<Vec<StockInfo>> {
        let data: Vec<StockInfo> = self.transport.get(point, None::<&StockCode>).await?;
        Ok(data)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StockInfo {
    #[serde(alias = "证券代码")]
    #[serde(alias = "A股代码")]
    pub code: String,
    #[serde(alias = "证券简称")]
    #[serde(alias = "A股简称")]
    pub name: String,
    #[serde(alias = "上市日期")]
    #[serde(alias = "A股上市日期")]
    pub time: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StockCode {
    symbol: String,
}

impl From<&str> for StockCode {
    fn from(s: &str) -> Self {
        Self {
            symbol: s.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<NaiveDate> for Quarter {
    fn from(s: NaiveDate) -> Self {
        match s.month() {
            1..=3 => Quarter::Q1,
            4..=6 => Quarter::Q2,
            7..=9 => Quarter::Q3,
            10.. => Quarter::Q4,
            _ => Quarter::Q4,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ReportDate {
    pub year: i32,
    pub quarter: Quarter,
}

impl ReportDate {
    pub fn new(y: i32, q: Quarter) -> ReportDate {
        ReportDate {
            year: y,
            quarter: q,
        }
    }
}

impl From<NaiveDate> for ReportDate {
    fn from(s: NaiveDate) -> Self {
        ReportDate {
            year: s.year(),
            quarter: s.into(),
        }
    }
}

impl fmt::Display for ReportDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let quart_str = match self.quarter {
            Quarter::Q1 => "03-31",
            Quarter::Q2 => "06-30",
            Quarter::Q3 => "09-30",
            Quarter::Q4 => "12-31",
        };
        write!(f, "{}-{}", self.year, quart_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sh_list_works() {
        let akshare = Akshare::new();
        if let Ok(client) = akshare {
            let s = client.sh_list().await.unwrap();
            print!("{:?}", s);
        }
    }

    #[tokio::test]
    async fn sz_list_works() {
        let akshare = Akshare::new();
        if let Ok(client) = akshare {
            let s = client.sz_list().await.unwrap();
            print!("{:?}", s);
        }
    }

    #[test]
    fn it_works() {
        let num = -2273;

        let t = "	SECUCODE: 002027.SZ,
        SECURITY_CODE: 002027,
        SECURITY_NAME_ABBR: 分众传媒,
        ORG_CODE: 10007267,
        ORG_TYPE: 通用,
        REPORT_DATE: 2022-06-30 00:00:00,
        REPORT_TYPE: 中报,
        REPORT_DATE_NAME: 2022中报,
        SECURITY_TYPE_CODE: 058001001,
        NOTICE_DATE: 2022-08-17 00:00:00,
        UPDATE_DATE: 2022-08-17 00:00:00,
        CURRENCY: CNY,
        SALES_SERVICES: 5681425638.07,
        DEPOSIT_INTERBANK_ADD: null,
        LOAN_PBC_ADD: null,
        OFI_BF_ADD: null,
        RECEIVE_ORIGIC_PREMIUM: null,
        RECEIVE_REINSURE_NET: null,
        INSURED_INVEST_ADD: null,
        DISPOSAL_TFA_ADD: null,
        RECEIVE_INTEREST_COMMISSION: null,
        BORROW_FUND_ADD: null,
        LOAN_ADVANCE_REDUCE: null,
        REPO_BUSINESS_ADD: null,
        RECEIVE_TAX_REFUND: null,
        RECEIVE_OTHER_OPERATE: 365405960.43,
        OPERATE_INFLOW_OTHER: null,
        OPERATE_INFLOW_BALANCE: 0.0,
        TOTAL_OPERATE_INFLOW: 6046831598.5,
        BUY_SERVICES: 140314986.69,
        LOAN_ADVANCE_ADD: null,
        PBC_INTERBANK_ADD: null,
        PAY_ORIGIC_COMPENSATE: null,
        PAY_INTEREST_COMMISSION: null,
        PAY_POLICY_BONUS: null,
        PAY_STAFF_CASH: 698670685.96,
        PAY_ALL_TAX: 1352052167.73,
        PAY_OTHER_OPERATE: 1179289920.26,
        OPERATE_OUTFLOW_OTHER: null,
        OPERATE_OUTFLOW_BALANCE: 0.0,
        TOTAL_OPERATE_OUTFLOW: 3370327760.64,
        OPERATE_NETCASH_OTHER: null,
        OPERATE_NETCASH_BALANCE: 0.0,
        NETCASH_OPERATE: 2676503837.86,
        WITHDRAW_INVEST: 214173205.87,
        RECEIVE_INVEST_INCOME: 70764545.69,
        DISPOSAL_LONG_ASSET: 396652.53,
        DISPOSAL_SUBSIDIARY_OTHER: 0.0,
        REDUCE_PLEDGE_TIMEDEPOSITS: null,
        RECEIVE_OTHER_INVEST: 6366290000.0,
        INVEST_INFLOW_OTHER: null,
        INVEST_INFLOW_BALANCE: 0.0,
        TOTAL_INVEST_INFLOW: 6651624404.09,
        CONSTRUCT_LONG_ASSET: 33562887.57,
        INVEST_PAY_CASH: 42800000.0,
        PLEDGE_LOAN_ADD: null,
        OBTAIN_SUBSIDIARY_OTHER: null,
        ADD_PLEDGE_TIMEDEPOSITS: null,
        PAY_OTHER_INVEST: 8210300000.0,
        INVEST_OUTFLOW_OTHER: null,
        INVEST_OUTFLOW_BALANCE: 0.0,
        TOTAL_INVEST_OUTFLOW: 8286662887.57,
        INVEST_NETCASH_OTHER: null,
        INVEST_NETCASH_BALANCE: 0.0,
        NETCASH_INVEST: -1635038483.48,
        ACCEPT_INVEST_CASH: 2067919.77,
        SUBSIDIARY_ACCEPT_INVEST: 2067919.77,
        RECEIVE_LOAN_CASH: 0.0,
        ISSUE_BOND: null,
        RECEIVE_OTHER_FINANCE: 0.0,
        FINANCE_INFLOW_OTHER: null,
        FINANCE_INFLOW_BALANCE: 0.0,
        TOTAL_FINANCE_INFLOW: 2067919.77,
        PAY_DEBT_CASH: 4411498.18,
        ASSIGN_DIVIDEND_PORFIT: 22500900.31,
        SUBSIDIARY_PAY_DIVIDEND: 22108867.76,
        BUY_SUBSIDIARY_EQUITY: null,
        PAY_OTHER_FINANCE: 1252537123.94,
        SUBSIDIARY_REDUCE_CASH: null,
        FINANCE_OUTFLOW_OTHER: null,
        FINANCE_OUTFLOW_BALANCE: 0.0,
        TOTAL_FINANCE_OUTFLOW: 1279449522.43,
        FINANCE_NETCASH_OTHER: null,
        FINANCE_NETCASH_BALANCE: 0.0,
        NETCASH_FINANCE: -1277381602.66,
        RATE_CHANGE_EFFECT: 8600562.89,
        CCE_ADD_OTHER: null,
        CCE_ADD_BALANCE: 0.0,
        CCE_ADD: -227315685.39,
        BEGIN_CCE: 4147015092.5,
        END_CCE_OTHER: null,
        END_CCE_BALANCE: 0.0,
        END_CCE: 3919699407.11,
        NETPROFIT: 1430931533.54,
        ASSET_IMPAIRMENT: 6454376.0,
        FA_IR_DEPR: 203409683.91,
        OILGAS_BIOLOGY_DEPR: 203409683.91,
        IR_DEPR: null,
        IA_AMORTIZE: 2771435.68,
        LPE_AMORTIZE: 3294513.2,
        DEFER_INCOME_AMORTIZE: null,
        PREPAID_EXPENSE_REDUCE: null,
        ACCRUED_EXPENSE_ADD: null,
        DISPOSAL_LONGASSET_LOSS: 519673.64,
        FA_SCRAP_LOSS: 105610.24,
        FAIRVALUE_CHANGE_LOSS: 81600311.33,
        FINANCE_EXPENSE: 53556002.4,
        INVEST_LOSS: -246987075.4,
        DEFER_TAX: -60249323.11,
        DT_ASSET_REDUCE: -37689231.09,
        DT_LIAB_ADD: -22560092.02,
        PREDICT_LIAB_ADD: null,
        INVENTORY_REDUCE: -3631957.35,
        OPERATE_RECE_REDUCE: 386755847.71,
        OPERATE_PAYABLE_ADD: -785821585.46,
        OTHER: null,
        OPERATE_NETCASH_OTHERNOTE: null,
        OPERATE_NETCASH_BALANCENOTE: 0.0,
        NETCASH_OPERATENOTE: 2676503837.86,
        DEBT_TRANSFER_CAPITAL: null,
        CONVERT_BOND_1YEAR: null,
        FINLEASE_OBTAIN_FA: null,
        UNINVOLVE_INVESTFIN_OTHER: null,
        END_CASH: 3919699407.11,
        BEGIN_CASH: 4147015092.5,
        END_CASH_EQUIVALENTS: null,
        BEGIN_CASH_EQUIVALENTS: null,
        CCE_ADD_OTHERNOTE: null,
        CCE_ADD_BALANCENOTE: null,
        CCE_ADDNOTE: -227315685.39,
        SALES_SERVICES_YOY: -29.8206079493,
        DEPOSIT_INTERBANK_ADD_YOY: null,
        LOAN_PBC_ADD_YOY: null,
        OFI_BF_ADD_YOY: null,
        RECEIVE_ORIGIC_PREMIUM_YOY: null,
        RECEIVE_REINSURE_NET_YOY: null,
        INSURED_INVEST_ADD_YOY: null,
        DISPOSAL_TFA_ADD_YOY: null,
        RECEIVE_INTEREST_COMMISSION_YOY: null,
        BORROW_FUND_ADD_YOY: null,
        LOAN_ADVANCE_REDUCE_YOY: null,
        REPO_BUSINESS_ADD_YOY: null,
        RECEIVE_TAX_REFUND_YOY: null,
        RECEIVE_OTHER_OPERATE_YOY: 17.0777320386,
        OPERATE_INFLOW_OTHER_YOY: null,
        OPERATE_INFLOW_BALANCE_YOY: null,
        TOTAL_OPERATE_INFLOW_YOY: -28.0796728661,
        BUY_SERVICES_YOY: -72.1103835691,
        LOAN_ADVANCE_ADD_YOY: null,
        PBC_INTERBANK_ADD_YOY: null,
        PAY_ORIGIC_COMPENSATE_YOY: null,
        PAY_INTEREST_COMMISSION_YOY: null,
        PAY_POLICY_BONUS_YOY: null,
        PAY_STAFF_CASH_YOY: 4.7944367804,
        PAY_ALL_TAX_YOY: -2.7892539214,
        PAY_OTHER_OPERATE_YOY: -34.8654201074,
        OPERATE_OUTFLOW_OTHER_YOY: null,
        OPERATE_OUTFLOW_BALANCE_YOY: null,
        TOTAL_OPERATE_OUTFLOW_YOY: -22.8970409246,
        OPERATE_NETCASH_OTHER_YOY: null,
        OPERATE_NETCASH_BALANCE_YOY: null,
        NETCASH_OPERATE_YOY: -33.6920779208,
        WITHDRAW_INVEST_YOY: -69.366496468,
        RECEIVE_INVEST_INCOME_YOY: 89.6553785275,
        DISPOSAL_LONG_ASSET_YOY: -81.5554627116,
        DISPOSAL_SUBSIDIARY_OTHER_YOY: -100.0,
        REDUCE_PLEDGE_TIMEDEPOSITS_YOY: null,
        RECEIVE_OTHER_INVEST_YOY: 296.6634682484,
        INVEST_INFLOW_OTHER_YOY: null,
        INVEST_INFLOW_BALANCE_YOY: null,
        TOTAL_INVEST_INFLOW_YOY: 183.7376556791,
        CONSTRUCT_LONG_ASSET_YOY: -53.5995594159,
        INVEST_PAY_CASH_YOY: -95.3104809215,
        PLEDGE_LOAN_ADD_YOY: null,
        OBTAIN_SUBSIDIARY_OTHER_YOY: null,
        ADD_PLEDGE_TIMEDEPOSITS_YOY: null,
        PAY_OTHER_INVEST_YOY: 157.658873372,
        INVEST_OUTFLOW_OTHER_YOY: null,
        INVEST_OUTFLOW_BALANCE_YOY: null,
        TOTAL_INVEST_OUTFLOW_YOY: 98.6491584129,
        INVEST_NETCASH_OTHER_YOY: null,
        INVEST_NETCASH_BALANCE_YOY: null,
        NETCASH_INVEST_YOY: 10.5176934996,
        ACCEPT_INVEST_CASH_YOY: -29.5733735801,
        SUBSIDIARY_ACCEPT_INVEST_YOY: -29.5733735801,
        RECEIVE_LOAN_CASH_YOY: -100.0,
        ISSUE_BOND_YOY: null,
        RECEIVE_OTHER_FINANCE_YOY: -100.0,
        FINANCE_INFLOW_OTHER_YOY: null,
        FINANCE_INFLOW_BALANCE_YOY: null,
        TOTAL_FINANCE_INFLOW_YOY: -97.9782617014,
        PAY_DEBT_CASH_YOY: -92.7600956083,
        ASSIGN_DIVIDEND_PORFIT_YOY: 3983.3247862179,
        SUBSIDIARY_PAY_DIVIDEND_YOY: null,
        BUY_SUBSIDIARY_EQUITY_YOY: null,
        PAY_OTHER_FINANCE_YOY: -8.0260667632,
        SUBSIDIARY_REDUCE_CASH_YOY: null,
        FINANCE_OUTFLOW_OTHER_YOY: null,
        FINANCE_OUTFLOW_BALANCE_YOY: null,
        TOTAL_FINANCE_OUTFLOW_YOY: -10.1083051899,
        FINANCE_NETCASH_OTHER_YOY: null,
        FINANCE_NETCASH_BALANCE_YOY: null,
        NETCASH_FINANCE_YOY: 3.3047882528,
        RATE_CHANGE_EFFECT_YOY: 310.0854605185,
        CCE_ADD_OTHER_YOY: null,
        CCE_ADD_BALANCE_YOY: null,
        CCE_ADD_YOY: -125.7108415886,
        BEGIN_CCE_YOY: -8.6802833057,
        END_CCE_OTHER_YOY: null,
        END_CCE_BALANCE_YOY: null,
        END_CCE_YOY: -27.7518494849,
        NETPROFIT_YOY: -50.9193932674,
        ASSET_IMPAIRMENT_YOY: -84.3520630213,
        FA_IR_DEPR_YOY: -18.204464787,
        OILGAS_BIOLOGY_DEPR_YOY: -18.204464787,
        IR_DEPR_YOY: null,
        IA_AMORTIZE_YOY: 1.0887996041,
        LPE_AMORTIZE_YOY: -40.933213514,
        DEFER_INCOME_AMORTIZE_YOY: null,
        PREPAID_EXPENSE_REDUCE_YOY: null,
        ACCRUED_EXPENSE_ADD_YOY: null,
        DISPOSAL_LONGASSET_LOSS_YOY: 116.4719570121,
        FA_SCRAP_LOSS_YOY: -84.7369936784,
        FAIRVALUE_CHANGE_LOSS_YOY: 198.2919307568,
        FINANCE_EXPENSE_YOY: 14.0213050677,
        INVEST_LOSS_YOY: -3.1245889355,
        DEFER_TAX_YOY: -156.5550690932,
        DT_ASSET_REDUCE_YOY: -142.2344505587,
        DT_LIAB_ADD_YOY: -230.4503039041,
        PREDICT_LIAB_ADD_YOY: null,
        INVENTORY_REDUCE_YOY: 68.2543776973,
        OPERATE_RECE_REDUCE_YOY: 42.7322522122,
        OPERATE_PAYABLE_ADD_YOY: -13.0399051488,
        OTHER_YOY: null,
        OPERATE_NETCASH_OTHERNOTE_YOY: null,
        OPERATE_NETCASH_BALANCENOTE_YOY: null,
        NETCASH_OPERATENOTE_YOY: -33.6920779208,
        DEBT_TRANSFER_CAPITAL_YOY: null,
        CONVERT_BOND_1YEAR_YOY: null,
        FINLEASE_OBTAIN_FA_YOY: null,
        UNINVOLVE_INVESTFIN_OTHER_YOY: null,
        END_CASH_YOY: -27.7518494849,
        BEGIN_CASH_YOY: -8.6802833057,
        END_CASH_EQUIVALENTS_YOY: null,
        BEGIN_CASH_EQUIVALENTS_YOY: null,
        CCE_ADD_OTHERNOTE_YOY: null,
        CCE_ADD_BALANCENOTE_YOY: null,
        CCE_ADDNOTE_YOY: -125.7108415886,
        OPINION_TYPE: null,
        OSOPINION_TYPE: null,
        MINORITY_INTEREST: null,
        MINORITY_INTEREST_YOY: null";
        find(t, num);
    }

    fn find(text: &str, num: i32) {
        let s = text
            .lines()
            .filter(|x| x.contains(num.to_string().as_str()))
            .collect::<Vec<_>>();
        if s.is_empty() {
            let num_new = if num.is_negative() { num + 1 } else { num - 1 };
            find(text, num_new)
        } else {
            s.iter().for_each(|x| println!("{x}"))
        }
    }
}
