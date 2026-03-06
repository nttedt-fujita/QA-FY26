"""
受入検査 分析ダッシュボード（Phase 1 プロトタイプ）

使い方:
    streamlit run tools/incoming_inspection/dashboard.py
"""

import streamlit as st
import pandas as pd
import plotly.express as px
import plotly.graph_objects as go
from pathlib import Path

# データディレクトリ
DATA_DIR = Path(__file__).parent / "output"


def load_data():
    """CSVファイルを読み込む"""
    monthly = pd.read_csv(DATA_DIR / "月別サマリー.csv", encoding="utf-8-sig")
    category = pd.read_csv(DATA_DIR / "detail" / "月別×カテゴリ.csv", encoding="utf-8-sig")
    worker = pd.read_csv(DATA_DIR / "detail" / "月別×作業者.csv", encoding="utf-8-sig")
    product = pd.read_csv(DATA_DIR / "detail" / "月別×部品.csv", encoding="utf-8-sig")
    inspection = pd.read_csv(DATA_DIR / "detail" / "月別×検査内容.csv", encoding="utf-8-sig")
    return monthly, category, worker, product, inspection


def main():
    st.set_page_config(
        page_title="受入検査 分析ダッシュボード",
        page_icon="📊",
        layout="wide"
    )

    st.title("📊 受入検査 分析ダッシュボード")
    st.caption("Phase 1 プロトタイプ — Excel分析結果の可視化")

    # データ読み込み
    try:
        monthly, category, worker, product, inspection = load_data()
    except FileNotFoundError as e:
        st.error(f"データファイルが見つかりません: {e}")
        st.info("先に月別分析スクリプトを実行してください:\n`python tools/incoming_inspection/monthly_analysis.py`")
        return

    # 「不明」を除外したデータ
    monthly_valid = monthly[monthly["年月"] != "不明"].copy()

    # --- サマリー指標 ---
    st.header("📈 サマリー")
    col1, col2, col3, col4 = st.columns(4)

    total_records = monthly["件数"].sum()
    total_hours = monthly["総工数(時間)"].sum()
    avg_hours_per_month = monthly_valid["総工数(時間)"].mean()
    unique_products = product["品名"].nunique()

    col1.metric("総レコード数", f"{total_records:,}件")
    col2.metric("総工数", f"{total_hours:.1f}時間")
    col3.metric("月平均工数", f"{avg_hours_per_month:.1f}時間/月")
    col4.metric("品名種類数", f"{unique_products:,}種類")

    # --- 月別推移 ---
    st.header("📅 月別推移")

    fig_monthly = go.Figure()
    fig_monthly.add_trace(go.Bar(
        x=monthly_valid["年月"],
        y=monthly_valid["件数"],
        name="件数",
        yaxis="y",
        marker_color="steelblue"
    ))
    fig_monthly.add_trace(go.Scatter(
        x=monthly_valid["年月"],
        y=monthly_valid["総工数(時間)"],
        name="工数(時間)",
        yaxis="y2",
        mode="lines+markers",
        line=dict(color="coral", width=3),
        marker=dict(size=8)
    ))
    fig_monthly.update_layout(
        title="月別 件数・工数推移",
        xaxis=dict(title="年月", tickangle=45),
        yaxis=dict(title="件数", side="left"),
        yaxis2=dict(title="工数(時間)", side="right", overlaying="y"),
        legend=dict(x=0.01, y=0.99),
        height=400
    )
    st.plotly_chart(fig_monthly, use_container_width=True)

    # --- カテゴリ別・作業者別 ---
    st.header("🏷️ 内訳分析")
    col_left, col_right = st.columns(2)

    with col_left:
        # カテゴリ別集計
        cat_summary = category.groupby("カテゴリ").agg({
            "件数": "sum",
            "工数(時間)": "sum"
        }).reset_index()

        fig_cat = px.pie(
            cat_summary,
            values="工数(時間)",
            names="カテゴリ",
            title="カテゴリ別 工数比率",
            color_discrete_sequence=px.colors.qualitative.Set2
        )
        st.plotly_chart(fig_cat, use_container_width=True)

    with col_right:
        # 作業者別集計
        worker_summary = worker.groupby("作業者").agg({
            "件数": "sum",
            "工数(時間)": "sum"
        }).reset_index()
        worker_summary = worker_summary.sort_values("工数(時間)", ascending=True)

        fig_worker = px.bar(
            worker_summary,
            x="工数(時間)",
            y="作業者",
            orientation="h",
            title="作業者別 工数",
            color="工数(時間)",
            color_continuous_scale="Blues"
        )
        fig_worker.update_layout(showlegend=False, coloraxis_showscale=False)
        st.plotly_chart(fig_worker, use_container_width=True)

    # --- 品名別パレート ---
    st.header("📦 品名別パレート分析")

    # 品名別集計
    product_summary = product.groupby("品名").agg({
        "件数": "sum",
        "工数(時間)": "sum"
    }).reset_index()
    product_summary = product_summary.sort_values("工数(時間)", ascending=False)

    # 上位N件を表示
    top_n = st.slider("表示件数", 5, 30, 15)
    top_products = product_summary.head(top_n).copy()

    # 累積比率を計算
    total_hours_product = product_summary["工数(時間)"].sum()
    top_products["累積工数"] = top_products["工数(時間)"].cumsum()
    top_products["累積比率(%)"] = (top_products["累積工数"] / total_hours_product * 100).round(1)

    fig_pareto = go.Figure()
    fig_pareto.add_trace(go.Bar(
        x=top_products["品名"],
        y=top_products["工数(時間)"],
        name="工数(時間)",
        marker_color="steelblue"
    ))
    fig_pareto.add_trace(go.Scatter(
        x=top_products["品名"],
        y=top_products["累積比率(%)"],
        name="累積比率(%)",
        yaxis="y2",
        mode="lines+markers",
        line=dict(color="coral", width=2),
        marker=dict(size=6)
    ))
    # 80%ライン
    fig_pareto.add_hline(
        y=80, line_dash="dash", line_color="red",
        annotation_text="80%ライン", yref="y2"
    )
    fig_pareto.update_layout(
        title=f"品名別 工数パレート図（上位{top_n}件）",
        xaxis=dict(title="品名", tickangle=45),
        yaxis=dict(title="工数(時間)", side="left"),
        yaxis2=dict(title="累積比率(%)", side="right", overlaying="y", range=[0, 105]),
        legend=dict(x=0.01, y=0.99),
        height=500
    )
    st.plotly_chart(fig_pareto, use_container_width=True)

    # --- 検査内容別 ---
    st.header("🔍 検査内容別分析")

    inspection_summary = inspection.groupby("検査内容").agg({
        "件数": "sum",
        "工数(時間)": "sum"
    }).reset_index()
    inspection_summary["平均工数(分)"] = (
        inspection_summary["工数(時間)"] / inspection_summary["件数"] * 60
    ).round(1)
    inspection_summary = inspection_summary.sort_values("工数(時間)", ascending=False)

    col_insp1, col_insp2 = st.columns(2)

    with col_insp1:
        fig_insp = px.bar(
            inspection_summary.head(10),
            x="検査内容",
            y="工数(時間)",
            title="検査内容別 工数（上位10件）",
            color="工数(時間)",
            color_continuous_scale="Greens"
        )
        fig_insp.update_layout(
            xaxis_tickangle=45,
            showlegend=False,
            coloraxis_showscale=False
        )
        st.plotly_chart(fig_insp, use_container_width=True)

    with col_insp2:
        # 平均工数の比較
        fig_avg = px.bar(
            inspection_summary.head(10),
            x="検査内容",
            y="平均工数(分)",
            title="検査内容別 平均工数（上位10件）",
            color="平均工数(分)",
            color_continuous_scale="Oranges"
        )
        fig_avg.update_layout(
            xaxis_tickangle=45,
            showlegend=False,
            coloraxis_showscale=False
        )
        st.plotly_chart(fig_avg, use_container_width=True)

    # --- データテーブル ---
    st.header("📋 データテーブル")

    tab1, tab2, tab3 = st.tabs(["月別サマリー", "品名別", "検査内容別"])

    with tab1:
        st.dataframe(
            monthly_valid[["年月", "件数", "総工数(時間)", "最多担当者", "パレート指標(%)"]],
            use_container_width=True
        )

    with tab2:
        st.dataframe(
            product_summary.head(30),
            use_container_width=True
        )

    with tab3:
        st.dataframe(
            inspection_summary,
            use_container_width=True
        )

    # --- フッター ---
    st.divider()
    st.caption("Phase 1 プロトタイプ — Excelデータから自動生成")
    st.caption("データソース: 受入検査作業集計.xlsx（杉山さんの記録のみ）")


if __name__ == "__main__":
    main()
