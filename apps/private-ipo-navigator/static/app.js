const STEPS = [
  {title:"Your destination", eyebrow:"OWNER OBJECTIVE"},
  {title:"Business reality", eyebrow:"CURRENT STATE"},
  {title:"Financial preferences", eyebrow:"OWNER PREFERENCES"},
  {title:"Explore scenarios", eyebrow:"PLATFORM SCENARIOS"},
  {title:"Decision dashboard", eyebrow:"COMPARE & PREPARE"},
  {title:"Document locker", eyebrow:"CONTROLLED DATA ROOM"},
  {title:"Professional handoff", eyebrow:"NEXT BEST ACTION"}
];

let state, documents = [], scenarioIndex = 0, saveTimer;
const $ = s => document.querySelector(s);
const money = n => new Intl.NumberFormat("en-US",{style:"currency",currency:"USD",maximumFractionDigits:0}).format(n||0);
const pct = n => `${Math.round(n)}%`;
const esc = s => String(s??"").replace(/[&<>"']/g,m=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#039;"}[m]));

async function load(){
  [state,documents] = await Promise.all([
    fetch("/api/state").then(r=>r.json()),
    fetch("/api/documents").then(r=>r.json())
  ]);
  state.documentLocker ||= {checklist:{},sharingConfirmed:false};
  state.documentLocker.checklist ||= {};
  if (!state.business.purchaseMultiple) {
    const implied = state.business.ebitda > 0
      ? state.business.enterpriseValue / state.business.ebitda
      : 5;
    state.business.purchaseMultiple = Math.min(10, Math.max(2, Math.round(implied * 10) / 10));
  }
  state.business.enterpriseValue = Math.round(state.business.ebitda * state.business.purchaseMultiple);
  if (state.preferences.cashAtClosingPct == null) {
    const impliedPct = state.business.enterpriseValue > 0
      ? (state.preferences.minimumCash / state.business.enterpriseValue) * 100
      : 40;
    state.preferences.cashAtClosingPct = Math.min(80, Math.max(0, Math.round(impliedPct)));
  }
  state.preferences.minimumCash = Math.round(
    state.business.enterpriseValue * state.preferences.cashAtClosingPct / 100
  );
  render();
}
function save(show=false){
  clearTimeout(saveTimer);
  $("#saveStatus").textContent="Saving…";
  saveTimer=setTimeout(async()=>{
    await fetch("/api/state",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(state)});
    $("#saveStatus").textContent="Saved";
    if(show) toast("Progress saved");
  },250);
}
function toast(text){
  const t=$("#toast");t.textContent=text;t.classList.add("show");setTimeout(()=>t.classList.remove("show"),1800);
}
function val(path, fallback=""){
  return path.split(".").reduce((o,k)=>o?.[k],state) ?? fallback;
}
function setPath(path,value){
  const keys=path.split(".");let o=state;keys.slice(0,-1).forEach(k=>o=o[k]);o[keys.at(-1)]=value;
  if(path==="preferences.cashPreference"){
    const cashPreferencePercent={
      "Mostly cash at closing":80,
      "Balanced cash now + future income":50,
      "Mostly income over time":20
    };
    state.preferences.cashAtClosingPct=cashPreferencePercent[value] ?? state.preferences.cashAtClosingPct;
    state.preferences.minimumCash=Math.round(
      state.business.enterpriseValue*state.preferences.cashAtClosingPct/100
    );
  }
  if(path==="business.ebitda" || path==="business.purchaseMultiple"){
    state.business.enterpriseValue=Math.round((+state.business.ebitda||0)*(+state.business.purchaseMultiple||0));
    state.preferences.minimumCash=Math.round(
      state.business.enterpriseValue*(+state.preferences.cashAtClosingPct||0)/100
    );
  }
  if(path==="preferences.cashAtClosingPct"){
    state.preferences.minimumCash=Math.round(
      state.business.enterpriseValue*(+state.preferences.cashAtClosingPct||0)/100
    );
  }
  save();render(false);
}
function toggle(path,value,max=99){
  const keys=path.split(".");let o=state;keys.slice(0,-1).forEach(k=>o=o[k]);
  const key=keys.at(-1), arr=o[key]||[], i=arr.indexOf(value);
  if(i>=0) arr.splice(i,1); else if(arr.length<max) arr.push(value);
  o[key]=arr;save();render(false);
}
function completeAndNext(){
  if(!state.completedSteps.includes(state.currentStep)) state.completedSteps.push(state.currentStep);
  if(state.currentStep<STEPS.length-1) state.currentStep++;
  save(true);render();window.scrollTo({top:0,behavior:"smooth"});
}
function nav(i){state.currentStep=i;save();render();window.scrollTo({top:0,behavior:"smooth"});}
function choice(path, value, title, sub="", icon=""){
  const selected=val(path)===value;
  return `<button class="choice ${selected?"selected":""}" data-set="${path}" data-value="${esc(value)}">
    ${icon?`<div class="icon">${icon}</div>`:""}<strong>${title}</strong>${sub?`<span>${sub}</span>`:""}</button>`;
}
function pill(path,value,danger=false){
  const selected=(val(path,[])||[]).includes(value);
  return `<button class="pill ${danger?"danger":""} ${selected?"selected":""}" data-toggle="${path}" data-value="${esc(value)}">${esc(value)}</button>`;
}
function input(path,label,type="text",opts={}){
  const value=val(path);
  return `<div class="field"><label>${label}</label><div class="${opts.money?"money-wrap":""}">
    ${opts.money?"<span>$</span>":""}<input data-input="${path}" type="${type}" value="${esc(value)}" ${opts.min!=null?`min="${opts.min}"`:""} ${opts.max!=null?`max="${opts.max}"`:""} ${opts.step?`step="${opts.step}"`:""} /></div>
    ${opts.help?`<div class="helper">${opts.help}</div>`:""}</div>`;
}
function formatSliderValue(value,format){
  const n=+value||0;
  if(format==="money") return money(n);
  if(format==="percent") return `${Number.isInteger(n)?n:n.toFixed(2)}%`;
  if(format==="months") return `${Math.round(n)} months`;
  if(format==="years") return `${Math.round(n)} years`;
  if(format==="employees") return `${Math.round(n).toLocaleString()} employees`;
  return n.toLocaleString();
}
function slider(path,label,opts={}){
  const value=+val(path,opts.min||0);
  const format=opts.format||"number";
  return `<div class="slider-field ${opts.full?"span-2":""}">
    <div class="slider-heading"><label>${label}</label><output data-slider-output="${path}">${formatSliderValue(value,format)}</output></div>
    <div class="slider-endpoints"><span>${opts.minLabel||formatSliderValue(opts.min,format)}</span><span>${opts.maxLabel||formatSliderValue(opts.max,format)}</span></div>
    <input class="assumption-slider" data-input="${path}" data-generic-slider="true" data-format="${format}" type="range" min="${opts.min}" max="${opts.max}" step="${opts.step||1}" value="${value}" />
    ${opts.help?`<div class="helper">${opts.help}</div>`:""}
  </div>`;
}
function select(path,label,items,help=""){
  return `<div class="field"><label>${label}</label><select data-input="${path}">${items.map(x=>`<option ${val(path)===x?"selected":""}>${x}</option>`).join("")}</select>${help?`<div class="helper">${help}</div>`:""}</div>`;
}
function render(scroll=false){
  $("#journeyNav").innerHTML=STEPS.map((s,i)=>`<button class="nav-step ${state.currentStep===i?"active":""} ${state.completedSteps.includes(i)?"done":""}" data-nav="${i}">
    <span class="num">${state.completedSteps.includes(i)?"✓":i+1}</span><span>${s.title}</span></button>`).join("");
  const meta=STEPS[state.currentStep];
  $("#pageTitle").textContent=meta.title;$("#stepEyebrow").textContent=meta.eyebrow;
  $("#progressLabel").textContent=`Step ${state.currentStep+1} of ${STEPS.length}`;
  $("#progressBar").style.width=`${((state.currentStep+1)/STEPS.length)*100}%`;
  $("#backBtn").style.visibility=state.currentStep?"visible":"hidden";
  $("#nextBtn").innerHTML=state.currentStep===STEPS.length-1?'Save my plan <span>✓</span>':'Continue <span>→</span>';
  $("#content").innerHTML=[destinationView,businessView,financeView,scenariosView,dashboardView,lockerView,handoffView][state.currentStep]();
  bind();
}
function bind(){
  document.querySelectorAll("[data-nav]").forEach(el=>el.onclick=()=>nav(+el.dataset.nav));
  document.querySelectorAll("[data-set]").forEach(el=>el.onclick=()=>setPath(el.dataset.set,el.dataset.number!==undefined?+el.dataset.number:el.dataset.value));
  document.querySelectorAll("[data-toggle]").forEach(el=>el.onclick=()=>toggle(el.dataset.toggle,el.dataset.value,+(el.dataset.max||99)));
  document.querySelectorAll("[data-input]").forEach(el=>{
    const commit=()=>{
      const v=(el.type==="number"||el.type==="range")?+el.value:el.value;
      setPath(el.dataset.input,v);
    };
    if(el.type==="range" && el.dataset.input==="business.purchaseMultiple"){
      el.oninput=()=>{
        state.business.purchaseMultiple=+el.value;
        state.business.enterpriseValue=Math.round((+state.business.ebitda||0)*state.business.purchaseMultiple);
        state.preferences.minimumCash=Math.round(
          state.business.enterpriseValue*(+state.preferences.cashAtClosingPct||0)/100
        );
        const multipleLabel=document.querySelector(".multiple-summary>div:nth-child(3) strong");
        const priceLabel=document.querySelector(".purchase-output strong");
        if(multipleLabel) multipleLabel.textContent=`${state.business.purchaseMultiple.toFixed(1)}×`;
        if(priceLabel) priceLabel.textContent=money(state.business.enterpriseValue);
        document.querySelectorAll(".multiple-pill").forEach(pill=>{
          pill.classList.toggle("selected",Math.abs(+pill.dataset.number-state.business.purchaseMultiple)<0.01);
        });
        save();
      };
      el.onchange=commit;
    } else if(el.type==="range" && el.dataset.input==="preferences.cashAtClosingPct"){
      el.oninput=()=>{
        state.preferences.cashAtClosingPct=+el.value;
        state.preferences.minimumCash=Math.round(
          state.business.enterpriseValue*state.preferences.cashAtClosingPct/100
        );
        const pctLabel=document.querySelector("#cashPctLabel");
        const amountLabel=document.querySelector("#cashAmountLabel");
        const banner=document.querySelector(".cash-closing-card .assumption-banner");
        if(pctLabel) pctLabel.textContent=`${Math.round(state.preferences.cashAtClosingPct)}%`;
        if(amountLabel) amountLabel.textContent=money(state.preferences.minimumCash);
        if(banner) banner.textContent=`The remaining ${Math.max(0,100-state.preferences.cashAtClosingPct)}% of purchase price would need to be modeled through seller financing or other deferred sources. This is exploratory—not a financing commitment.`;
        save();
      };
      el.onchange=commit;
    } else if(el.type==="range" && el.dataset.genericSlider==="true"){
      el.oninput=()=>{
        const value=+el.value;
        const keys=el.dataset.input.split(".");
        let target=state;
        keys.slice(0,-1).forEach(k=>target=target[k]);
        target[keys.at(-1)]=value;
        const output=document.querySelector(`[data-slider-output="${el.dataset.input}"]`);
        if(output) output.textContent=formatSliderValue(value,el.dataset.format);
        save();
      };
      el.onchange=commit;
    } else {
      el.onchange=commit;
    }
  });
  document.querySelectorAll("[data-scenario]").forEach(el=>el.onclick=()=>{scenarioIndex=+el.dataset.scenario;render(false)});
  document.querySelectorAll("[data-select-scenario]").forEach(el=>el.onclick=()=>{state.selectedScenario=el.dataset.selectScenario;save(true);render(false)});
  document.querySelectorAll("[data-locker-item]").forEach(el=>el.onclick=()=>{
    const id=el.dataset.lockerItem;
    const statuses=["Missing","Requested","Ready"];
    const current=state.documentLocker.checklist[id]||"Missing";
    state.documentLocker.checklist[id]=statuses[(statuses.indexOf(current)+1)%statuses.length];
    save();render(false);
  });
  const uploadForm=document.querySelector("#lockerUploadForm");
  if(uploadForm) uploadForm.onsubmit=uploadDocument;
  const lockerFile=document.querySelector("#lockerFile");
  if(lockerFile) lockerFile.onchange=()=>{
    const label=document.querySelector("#fileChoice");
    if(label) label.textContent=lockerFile.files[0]?.name||"Choose a file";
  };
  document.querySelectorAll("[data-doc-delete]").forEach(el=>el.onclick=()=>deleteDocument(el.dataset.docDelete));
  document.querySelectorAll("[data-doc-category],[data-doc-sensitivity]").forEach(el=>el.onchange=()=>updateDocumentMeta(el.dataset.docCategory||el.dataset.docSensitivity));
  document.querySelectorAll("[data-doc-recipient]").forEach(el=>el.onchange=()=>updateDocumentMeta(el.dataset.docRecipient));
}

function destinationNarrative(){
  const d=state.destination, p=state.preferences;
  const priorities=d.priorities.length?d.priorities.slice(0,3).join(", ").toLowerCase():"preserve what matters";
  return `When this transition is complete, I want my employees to own 100% of the company. I want to ${d.ownerRole.toLowerCase()} on a ${d.timing.toLowerCase()} timeline, while prioritizing ${priorities}. ${p.minimumCash?`I would like to explore at least ${money(p.minimumCash)} of cash at closing.`:""}`;
}
function destinationView(){
  const priorities=["Preserve jobs","Protect company culture","Keep the company independent","Meaningful employee ownership","Receive more cash at closing","Create future income","Retire completely","Remain involved","Protect family / estate interests"];
  const avoids=["Selling to an outside strategic buyer","Remaining involved long-term","Excessive company debt","Waiting many years for proceeds","Disrupting employees","Moving the company","Losing control too quickly"];
  return `<div class="hero-copy"><h2>Start with the ending you want.</h2><p>Before discussing ESOPs, financing or deal structures, define what success should feel like—for you, your employees and the business.</p></div>
  <div class="section-title"><h3>Your north star</h3><p>The employee-ownership outcome is fixed; the path is still open.</p></div>
  <div class="choice-grid">
    ${choice("destination.primaryGoal","100% employee ownership","Employees own 100%","The ultimate ownership destination.","◎")}
    ${choice("destination.primaryGoal","Employee ownership with flexibility","Employee ownership + flexibility","Explore staged or mixed paths.","◐")}
    ${choice("destination.primaryGoal","Help me explore","Help me explore","Keep the destination open for now.","◇")}
  </div>
  <div class="section-title"><h3>What else matters?</h3><p>Select the outcomes you want the transaction to protect.</p></div>
  <div class="pills">${priorities.map(x=>pill("destination.priorities",x)).join("")}</div>
  <div class="section-title"><h3>What must the path avoid?</h3><p>Negative constraints are just as important as positive goals.</p></div>
  <div class="pills">${avoids.map(x=>pill("destination.avoid",x,true)).join("")}</div>
  <div class="section-title"><h3>Timing and your future role</h3></div>
  <div class="grid-2">
    <div class="card"><h4>Ideal transition timing</h4><div class="choice-grid" style="grid-template-columns:1fr 1fr;margin-top:14px">
      ${["Within 1 year","1–3 years","3–5 years","Flexible"].map(x=>choice("destination.timing",x,x)).join("")}</div></div>
    <div class="card"><h4>Your role after the transaction</h4><div class="choice-grid" style="grid-template-columns:1fr 1fr;margin-top:14px">
      ${["Retire completely","Temporary transition advisor","Remain involved","Not sure"].map(x=>choice("destination.ownerRole",x,x)).join("")}</div></div>
  </div>
  <div class="section-title"><h3>Your destination, in plain language</h3></div>
  <div class="card destination-card"><div class="eyebrow" style="color:#cfe566">DRAFT DESTINATION</div><p>“${destinationNarrative()}”</p>
    <div class="destination-meta"><span>Owner-stated</span><span>Editable</span><span>Not a transaction recommendation</span></div></div>`;
}
function businessView(){
  return `<div class="hero-copy"><h2>Now, describe the business today.</h2><p>Start with approximate numbers. These owner-reported inputs help surface relevant paths; they are not a valuation or a feasibility determination.</p></div>
  <div class="card">
    <div class="fields">
      ${input("business.companyName","Company name")}
      ${input("business.industry","Industry")}
      ${input("business.ebitda","Estimated EBITDA","number",{money:true,min:0,step:50000,help:"Keep this as a direct input so you can enter the estimate precisely."})}
      ${slider("business.revenue","Approximate annual revenue",{min:0,max:100000000,step:250000,format:"money",minLabel:"$0",maxLabel:"$100M",help:"Use your most recent representative year."})}
      ${slider("business.employees","Number of employees",{min:1,max:1000,step:1,format:"employees",minLabel:"1",maxLabel:"1,000+"})}
      ${slider("business.existingDebt","Existing business debt",{min:0,max:Math.max(5000000,state.business.enterpriseValue),step:50000,format:"money",minLabel:"$0",maxLabel:money(Math.max(5000000,state.business.enterpriseValue))})}
      ${slider("business.transactionCosts","Estimated fees and transaction costs",{min:0,max:Math.max(1000000,Math.round(state.business.enterpriseValue*.1)),step:25000,format:"money",minLabel:"$0",maxLabel:money(Math.max(1000000,Math.round(state.business.enterpriseValue*.1)))})}
    </div>
  </div>
  <div class="section-title"><h3>Desired purchase price</h3><p>Choose an EBITDA multiple. The purchase price updates automatically.</p></div>
  <div class="card multiple-card">
    <div class="multiple-summary">
      <div><span>Estimated EBITDA</span><strong>${money(state.business.ebitda)}</strong></div>
      <div class="formula-mark">×</div>
      <div><span>Purchase multiple</span><strong>${(+state.business.purchaseMultiple).toFixed(1)}×</strong></div>
      <div class="formula-mark">=</div>
      <div class="purchase-output"><span>Desired purchase price</span><strong>${money(state.business.enterpriseValue)}</strong></div>
    </div>
    <div class="multiple-control">
      <div class="range-labels"><span>2×</span><b>Slide to explore purchase price</b><span>10×</span></div>
      <input class="multiple-slider" data-input="business.purchaseMultiple" type="range" min="2" max="10" step="0.1" value="${state.business.purchaseMultiple}" />
      <div class="quick-multiples">
        <span>Quick select</span>
        ${[3.5,4.5,5.5].map(x=>`<button class="pill multiple-pill ${Math.abs(state.business.purchaseMultiple-x)<0.01?"selected":""}" data-set="business.purchaseMultiple" data-number="${x}">${x.toFixed(1)}× EBITDA</button>`).join("")}
      </div>
    </div>
    <div class="assumption-banner">This is an owner-selected purchase-price assumption for scenario exploration—not a valuation opinion. An independent valuation professional must determine supportable value.</div>
  </div>
  <div class="section-title"><h3>Transition readiness signals</h3><p>These conditions may shape the work required before an ownership transition.</p></div>
  <div class="grid-2">
    <div class="card">${select("business.ownerDependency","How dependent is the business on you?",["Business operates independently","Team could take over","Several critical functions depend on me","Almost everything depends on me","Not sure"])}</div>
    <div class="card">${select("business.managementReadiness","Management leadership coverage",["Strong","Some","Not yet","Not sure"])}</div>
    <div class="card">${select("business.employeeInterest","Employee interest in ownership",["Very interested","Some interest","Not yet discussed","Not sure"])}</div>
    <div class="card">${select("business.customerConcentration","Customer concentration",["Low","Moderate","High","Not sure"])}</div>
  </div>
  <div class="section-title"><h3>Destination vs. reality</h3></div>
  <div class="snapshot-grid">
    <div class="snapshot want"><div class="label">WHERE YOU WANT TO GO</div><ul>
      <li><span>Ownership</span><b>${esc(state.destination.primaryGoal)}</b></li><li><span>Timing</span><b>${esc(state.destination.timing)}</b></li><li><span>Owner role</span><b>${esc(state.destination.ownerRole)}</b></li>
    </ul></div>
    <div class="snapshot today"><div class="label">WHERE THE BUSINESS IS TODAY</div><ul>
      <li><span>Revenue</span><b>${money(state.business.revenue)}</b></li><li><span>EBITDA</span><b>${money(state.business.ebitda)}</b></li><li><span>Employees</span><b>${state.business.employees}</b></li><li><span>Owner dependency</span><b>${esc(state.business.ownerDependency)}</b></li>
    </ul></div>
  </div>`;
}
function financeView(){
  return `<div class="hero-copy"><h2>What financial outcome are you trying to create?</h2><p>Tell us what matters. The scenarios will show the arithmetic and trade-offs without claiming the business is worth a specific amount.</p></div>
  <div class="section-title"><h3>How would you like to receive the value?</h3></div>
  <div class="choice-grid">
    ${choice("preferences.cashPreference","Mostly cash at closing","Mostly cash now","Sets cash at closing to 80% of desired purchase price.","$")}
    ${choice("preferences.cashPreference","Balanced cash now + future income","Balanced","Sets cash at closing to 50% with the remainder deferred.","◒")}
    ${choice("preferences.cashPreference","Mostly income over time","Income over time","Sets cash at closing to 20% and models more deferred value.","↗")}
  </div>
  <div class="section-title"><h3>Exploration assumptions</h3><p>These assumptions are connected to your desired purchase price and update automatically.</p></div>
  <div class="card cash-closing-card">
    <div class="cash-closing-summary">
      <div><span>Desired purchase price</span><strong>${money(state.business.enterpriseValue)}</strong></div>
      <div class="formula-mark">×</div>
      <div><span>Cash at closing</span><strong id="cashPctLabel">${Math.round(state.preferences.cashAtClosingPct)}%</strong></div>
      <div class="formula-mark">=</div>
      <div class="purchase-output"><span>Target cash at closing</span><strong id="cashAmountLabel">${money(state.preferences.minimumCash)}</strong></div>
    </div>
    <div class="multiple-control">
      <div class="range-labels"><span>0%</span><b>Cash at closing as a share of desired purchase price</b><span>80%</span></div>
      <input class="multiple-slider cash-slider" data-input="preferences.cashAtClosingPct" type="range" min="0" max="80" step="1" value="${state.preferences.cashAtClosingPct}" />
    </div>
    <div class="assumption-banner">The remaining ${Math.max(0,100-state.preferences.cashAtClosingPct)}% of purchase price would need to be modeled through seller financing or other deferred sources. This is exploratory—not a financing commitment.</div>
  </div>
  <div class="card" style="margin-top:16px">
    <div class="fields slider-fields">
      ${slider("preferences.maxTransitionMonths","Maximum owner transition",{min:0,max:120,step:3,format:"months",minLabel:"0 months",maxLabel:"10 years"})}
      ${slider("preferences.sellerNoteRate","Illustrative seller-note rate",{min:0,max:20,step:.25,format:"percent",minLabel:"0%",maxLabel:"20%"})}
      ${slider("preferences.sellerNoteTerm","Illustrative seller-note term",{min:1,max:20,step:1,format:"years",minLabel:"1 year",maxLabel:"20 years",full:true})}
    </div>
  </div>
  <div class="section-title"><h3>Comfort with uncertainty</h3></div>
  <div class="choice-grid">
    ${choice("preferences.riskComfort","Lower uncertainty","Lower uncertainty","Favor clarity and earlier liquidity.")}
    ${choice("preferences.riskComfort","Balanced","Balanced","Balance immediate proceeds and future upside.")}
    ${choice("preferences.riskComfort","More future participation","More future participation","Accept more deferred value or staged execution.")}
  </div>
  <div class="callout" style="margin-top:22px"><strong>Why this matters</strong>Employee ownership is the destination. Financing choices determine how much liquidity arrives now, how much depends on future company performance, and how much debt the business may carry.</div>`;
}
function payment(principal, rate, years){
  if(principal<=0)return 0; const r=rate/100; return r?principal*(r*Math.pow(1+r,years))/(Math.pow(1+r,years)-1):principal/years;
}
function scenarios(){
  const b=state.business,p=state.preferences;
  const ev=Math.max(0,+b.enterpriseValue), net=Math.max(0,ev-b.existingDebt-b.transactionCosts);
  const configs=[
    {id:"direct",name:"Direct 100% ESOP sale",short:"Direct ESOP",tag:"EMPLOYEE TRUST AT CLOSING",bankPct:.48,notePct:.42,cashPct:null,years:"12–24 months",ownership:"100% at closing",involvement:"12–24 months",employee:"Broad-based beneficial ownership",complexity:"High",why:"Keeps the ownership destination direct while combining third-party debt and seller financing.",pros:["100% employee ownership at closing","Broad participation can be designed into the trust","Owner receives meaningful cash at close"],warn:["Requires independent valuation and specialized fiduciary review","Company carries acquisition-related obligations","Seller note depends on future company performance"]},
    {id:"bridge",name:"PE bridge → 100% employee ownership",short:"PE bridge",tag:"SPONSORED TRANSITION",bankPct:.72,notePct:.18,years:"3–5 years to 100%",ownership:"100% after second-stage sale",involvement:"6–18 months",employee:"Ownership delivered after bridge period",complexity:"Very high",why:"Uses an interim sponsor to provide more liquidity and prepare the company for a later employee buyout.",pros:["Potentially more cash to owner at initial closing","Adds transition capital and execution support","Can create time to reduce owner dependency"],warn:["Employees do not own 100% immediately","Second-stage sale and timing are not guaranteed","Two transactions add cost, governance and execution risk"]},
    {id:"staged",name:"Seller-sponsored 100% employee transition",short:"Seller-sponsored",tag:"MORE DEFERRED VALUE",bankPct:.32,notePct:.58,years:"12–30 months",ownership:"100% at closing",involvement:"18–30 months",employee:"Broad-based or direct structure",complexity:"Medium–high",why:"Reduces reliance on outside capital by using a larger seller note and a longer transition.",pros:["100% employee ownership can occur at closing","Potential for ongoing income to seller","Lower third-party acquisition debt"],warn:["Lower cash at initial closing","Greater seller-note exposure","May require longer owner involvement and stronger cash flow"]}
  ];
  return configs.map(c=>{
    const bank=ev*c.bankPct, note=ev*c.notePct, equity=Math.max(0,ev-bank-note);
    const cash=Math.max(0,bank+equity-b.existingDebt-b.transactionCosts);
    const annualSenior=payment(bank,8,7), annualNote=payment(note,p.sellerNoteRate,p.sellerNoteTerm);
    const coverage=b.ebitda/(annualSenior+annualNote||1);
    const conflicts=[];
    if(cash<p.minimumCash) conflicts.push(`Cash at closing is ${money(p.minimumCash-cash)} below your target.`);
    if(state.destination.nonnegotiables?.includes("100% employee ownership") && c.id==="bridge") conflicts.push("100% employee ownership is delayed during the bridge period.");
    if(c.id==="staged" && p.maxTransitionMonths<18) conflicts.push("Likely owner involvement may exceed your target.");
    return {...c,ev,bank,note,equity,cash,net,annualSenior,annualNote,coverage,conflicts};
  });
}
function scenarioPanel(s){
  return `<div class="card scenario-card">
    <div class="scenario-head"><div><div class="eyebrow">${s.tag}</div><h3>${s.name}</h3><p class="helper" style="max-width:700px">${s.why}</p></div><span class="tag">EXPLORATORY</span></div>
    <div class="scenario-body">
      <div class="metric-row">
        <div class="metric"><span>Modeled cash at close</span><strong>${money(s.cash)}</strong></div>
        <div class="metric"><span>Illustrative seller note</span><strong>${money(s.note)}</strong></div>
        <div class="metric"><span>Employee ownership</span><strong>${s.ownership}</strong></div>
        <div class="metric"><span>Modeled coverage</span><strong>${s.coverage.toFixed(2)}×</strong></div>
      </div>
      <div class="assumption-banner">Illustrative only: ${money(s.ev)} enterprise value, ${money(s.bank)} senior financing, ${money(s.note)} seller note, ${money(s.equity)} other/equity source. Professional valuation and financing review required.</div>
      <div class="fit-row">
        <div class="fit-box"><h4>What this path may support</h4><ul>${s.pros.map(x=>`<li><span class="check">✓</span> ${x}</li>`).join("")}</ul></div>
        <div class="fit-box"><h4>Trade-offs & questions</h4><ul>${s.warn.map(x=>`<li><span class="warn">△</span> ${x}</li>`).join("")}${s.conflicts.map(x=>`<li><span class="warn">⚠</span> <strong>${x}</strong></li>`).join("")}</ul></div>
      </div>
      <button class="primary" style="margin-top:18px" data-select-scenario="${s.id}">${state.selectedScenario===s.id?"Selected for professional review ✓":"Carry forward for review"}</button>
    </div></div>`;
}
function scenariosView(){
  const ss=scenarios(),s=ss[scenarioIndex]||ss[0];
  return `<div class="hero-copy"><div class="layer-badges"><span class="owner-layer">Owner objective</span><span class="platform-layer">Platform scenario</span><span class="pro-layer">Professional review pending</span></div>
  <h2>Three paths to the same destination.</h2><p>Each path aims toward 100% employee ownership, but shifts liquidity, timing and risk differently. This comparison does not declare a winner.</p></div>
  <div class="scenario-tabs">${ss.map((x,i)=>`<button data-scenario="${i}" class="scenario-tab ${i===scenarioIndex?"active":""}">${x.short}</button>`).join("")}</div>
  ${scenarioPanel(s)}
  <div class="section-title"><h3>Quick comparison</h3></div>
  ${comparisonTable(ss)}`;
}
function comparisonTable(ss){
  const row=(name,fn)=>`<tr><td>${name}</td>${ss.map(s=>`<td>${fn(s)}</td>`).join("")}</tr>`;
  return `<div class="card comparison"><table class="compare-table"><thead><tr><th>Dimension</th>${ss.map(s=>`<th>${s.short}</th>`).join("")}</tr></thead><tbody>
    ${row("Modeled cash at close",s=>money(s.cash))}
    ${row("Seller note",s=>money(s.note))}
    ${row("100% employee ownership",s=>s.ownership)}
    ${row("Owner transition",s=>s.involvement)}
    ${row("Company debt service",s=>money(s.annualSenior+s.annualNote)+"/yr")}
    ${row("Key unknown",s=>s.id==="direct"?"Independent valuation":s.id==="bridge"?"Second-stage timing":"Seller-note support")}
    ${row("Objective conflicts",s=>s.conflicts.length?`<span class="warn">${s.conflicts.length} to explore</span>`:`<span class="check">None identified</span>`)}
  </tbody></table></div>`;
}
function dashboardView(){
  const ss=scenarios(), selected=ss.find(s=>s.id===state.selectedScenario);
  const dataQuality=[state.business.companyName,state.business.industry,state.business.revenue,state.business.ebitda,state.business.enterpriseValue,state.business.employees,state.business.ownerDependency,state.business.managementReadiness].filter(Boolean).length/8;
  const readiness=Math.round(35+dataQuality*35+(state.selectedScenario?15:0)+(state.destination.priorities.length?10:0));
  return `<div class="hero-copy"><h2>Your private IPO decision dashboard.</h2><p>A single view of your destination, current assumptions and the questions that must be resolved before a transaction direction is chosen.</p></div>
  <div class="kpis">
    <div class="kpi"><small>Ownership destination</small><strong>100%</strong><div class="trend">Employee-owned</div></div>
    <div class="kpi"><small>Illustrative value</small><strong>${money(state.business.enterpriseValue)}</strong><div class="trend">Owner-entered assumption</div></div>
    <div class="kpi"><small>Target cash at close</small><strong>${money(state.preferences.minimumCash)}</strong><div class="trend">Owner objective</div></div>
    <div class="kpi"><small>Preparation progress</small><strong>${readiness}%</strong><div class="trend">Information completeness</div></div>
  </div>
  <div class="grid-3">
    <div class="card span-2 destination-card"><div class="eyebrow" style="color:#cfe566">YOUR NORTH STAR</div><p style="font-size:16px">“${destinationNarrative()}”</p></div>
    <div class="card" style="display:flex;align-items:center;gap:18px"><div class="readiness-ring" style="--pct:${readiness}%"><div><strong>${readiness}%</strong><span>PREPARED</span></div></div><div><h3>Exploration readiness</h3><p class="helper">Measures input completeness—not transaction feasibility.</p></div></div>
  </div>
  <div class="section-title"><h3>Scenario comparison</h3><p>No universal score; compare the dimensions that matter to you.</p></div>
  ${comparisonTable(ss)}
  <div class="section-title"><h3>Current working direction</h3></div>
  ${selected?scenarioPanel(selected):`<div class="card empty-state"><div class="big">◇</div><h3>No path carried forward yet</h3><p class="helper">Return to Explore scenarios and select one for professional review. You can change it later.</p><button class="primary" data-nav="3" style="margin-top:15px">Explore paths</button></div>`}`;
}

const LOCKER_ITEMS=[
  {id:"financials",category:"Financial",name:"3 years of financial statements",why:"Historical income statements, balance sheets and cash flow statements.",stakeholders:["Valuation advisor","Tax / CPA","Financing sources"]},
  {id:"interim",category:"Financial",name:"Current YTD and trailing-12-month financials",why:"Current operating performance and normalization review.",stakeholders:["Valuation advisor","Tax / CPA","Financing sources"]},
  {id:"tax",category:"Financial",name:"3 years of business tax returns",why:"Tax diligence and reconciliation to reported results.",stakeholders:["Tax / CPA","Valuation advisor"]},
  {id:"debt",category:"Financial",name:"Debt and lien schedule",why:"Existing obligations, maturity dates, rates and collateral.",stakeholders:["Financing sources","Legal counsel"]},
  {id:"census",category:"People",name:"Employee census and benefit data",why:"Eligibility and preliminary ESOP feasibility; redact personal identifiers until requested.",stakeholders:["ESOP advisor / trustee","Tax / CPA"]},
  {id:"org",category:"People",name:"Organization chart and management succession plan",why:"Leadership coverage and post-close operating readiness.",stakeholders:["ESOP advisor / trustee","Management / board"]},
  {id:"customers",category:"Commercial",name:"Customer concentration and revenue detail",why:"Revenue durability and concentration diligence.",stakeholders:["Valuation advisor","Financing sources"]},
  {id:"contracts",category:"Legal",name:"Material contracts and leases",why:"Change-of-control, assignment and obligation review.",stakeholders:["Legal counsel"]},
  {id:"ownership",category:"Legal",name:"Cap table and governing documents",why:"Ownership, authority and transaction approval diligence.",stakeholders:["Legal counsel","ESOP advisor / trustee"]},
  {id:"goals",category:"Transaction",name:"Owner objectives and scenario brief",why:"Aligns the professional team on the destination and assumptions.",stakeholders:["Core deal team"]},
  {id:"valuation",category:"Transaction",name:"Independent valuation materials",why:"Added by the qualified valuation professional when available.",stakeholders:["ESOP advisor / trustee","Legal counsel","Financing sources"]},
  {id:"financing",category:"Transaction",name:"Financing indications and sources & uses",why:"Documents funding capacity, terms and cash-at-close mechanics.",stakeholders:["Core deal team","Legal counsel"]}
];
const STAKEHOLDERS=["Core deal team","Valuation advisor","ESOP advisor / trustee","Legal counsel","Tax / CPA","Financing sources","Management / board","Employee communications"];
const CATEGORIES=["Financial","People","Commercial","Legal","Transaction","Other"];
const SENSITIVITY=["Confidential","Highly confidential","Restricted PII","Approved to share"];

function fileSize(bytes){
  if(bytes<1024) return `${bytes} B`;
  if(bytes<1024*1024) return `${(bytes/1024).toFixed(1)} KB`;
  return `${(bytes/1024/1024).toFixed(1)} MB`;
}
function lockerStatus(item){
  return state.documentLocker.checklist[item.id]||"Missing";
}
function lockerView(){
  const ready=LOCKER_ITEMS.filter(x=>lockerStatus(x)==="Ready").length;
  const requested=LOCKER_ITEMS.filter(x=>lockerStatus(x)==="Requested").length;
  const missing=LOCKER_ITEMS.length-ready-requested;
  const grouped=Object.groupBy?Object.groupBy(LOCKER_ITEMS,x=>x.category):LOCKER_ITEMS.reduce((a,x)=>((a[x.category]||=[]).push(x),a),{});
  const stakeholderRows=STAKEHOLDERS.map(stakeholder=>{
    const recommended=LOCKER_ITEMS.filter(x=>x.stakeholders.includes(stakeholder)||x.stakeholders.includes("Core deal team"));
    const uploads=documents.filter(d=>d.recipients.includes(stakeholder));
    return `<tr><td><strong>${stakeholder}</strong></td><td>${recommended.map(x=>x.name).slice(0,4).join(", ")}${recommended.length>4?"…":""}</td><td>${uploads.length?`${uploads.length} assigned`:"None assigned"}</td></tr>`;
  }).join("");
  return `<div class="hero-copy"><div class="layer-badges"><span class="owner-layer">Owner-controlled</span><span class="platform-layer">Organized & classified</span><span class="pro-layer">Shared only when approved</span></div>
    <h2>Prepare one controlled source of truth.</h2><p>Collect diligence materials, mark readiness and define who should receive each file. Nothing is sent automatically.</p></div>
  <div class="kpis locker-kpis">
    <div class="kpi"><small>Required checklist</small><strong>${LOCKER_ITEMS.length}</strong><div class="trend">Core diligence items</div></div>
    <div class="kpi"><small>Ready</small><strong>${ready}</strong><div class="trend">Marked ready to review</div></div>
    <div class="kpi"><small>Requested</small><strong>${requested}</strong><div class="trend">Being gathered</div></div>
    <div class="kpi"><small>Files uploaded</small><strong>${documents.length}</strong><div class="trend">${missing} checklist items missing</div></div>
  </div>
  <div class="locker-layout">
    <div>
      <div class="section-title"><h3>Diligence checklist</h3><p>Click a status to move an item from Missing → Requested → Ready.</p></div>
      ${Object.entries(grouped).map(([category,items])=>`<div class="card locker-group">
        <div class="locker-group-head"><h4>${category}</h4><span>${items.filter(x=>lockerStatus(x)==="Ready").length}/${items.length} ready</span></div>
        ${items.map(item=>`<button class="locker-check-row" data-locker-item="${item.id}">
          <span class="status-dot ${lockerStatus(item).toLowerCase()}"></span>
          <span class="locker-check-copy"><strong>${item.name}</strong><small>${item.why}</small></span>
          <span class="locker-status ${lockerStatus(item).toLowerCase()}">${lockerStatus(item)}</span>
        </button>`).join("")}
      </div>`).join("")}
    </div>
    <div>
      <div class="section-title"><h3>Add to the locker</h3><p>Files are private to your journey until you approve external sharing.</p></div>
      <form class="card upload-card" id="lockerUploadForm">
        <label class="upload-drop" for="lockerFile"><span class="upload-icon">↑</span><strong id="fileChoice">Choose a file</strong><small>PDF, spreadsheet, document, image or CSV · 25 MB max</small></label>
        <input id="lockerFile" type="file" hidden required accept=".pdf,.xlsx,.xls,.doc,.docx,.csv,.png,.jpg,.jpeg,.txt" />
        <div class="fields">
          <div class="field"><label>Category</label><select id="uploadCategory">${CATEGORIES.map(x=>`<option>${x}</option>`).join("")}</select></div>
          <div class="field"><label>Sensitivity</label><select id="uploadSensitivity">${SENSITIVITY.map(x=>`<option>${x}</option>`).join("")}</select></div>
        </div>
        <div class="recipient-picker"><label>Intended recipients</label>${STAKEHOLDERS.map(x=>`<label class="recipient-option"><input type="checkbox" name="uploadRecipient" value="${x}" /> <span>${x}</span></label>`).join("")}</div>
        <button class="primary full" type="submit" id="uploadButton">Add to private locker</button>
        <div class="upload-error" id="uploadError"></div>
      </form>
      <div class="section-title"><h3>Stored documents</h3><p>${documents.length?"Review classification and intended access.":"No files uploaded yet."}</p></div>
      <div class="document-list">${documents.length?documents.map(documentCard).join(""):`<div class="card empty-state compact"><div class="big">▣</div><h3>Your locker is empty</h3><p class="helper">Upload files or use the checklist to request them from your team.</p></div>`}</div>
    </div>
  </div>
  <div class="section-title"><h3>Stakeholder package map</h3><p>A preparation map—not a sharing action. Final access should follow counsel and deal-team instructions.</p></div>
  <div class="card comparison"><table class="compare-table locker-matrix"><thead><tr><th>Stakeholder</th><th>Recommended material</th><th>Uploaded files assigned</th></tr></thead><tbody>${stakeholderRows}</tbody></table></div>
  <div class="callout locker-privacy"><strong>Least-privilege sharing</strong>Employee census, tax records, personal data and financing materials should not be broadcast to every stakeholder. Confirm recipients, redaction and timing before release.</div>`;
}
function documentCard(doc){
  return `<div class="card document-card">
    <div class="document-title"><span class="document-icon">▤</span><div><strong>${esc(doc.filename)}</strong><small>${fileSize(doc.sizeBytes)} · added ${new Date(doc.createdAt+"Z").toLocaleDateString()}</small></div></div>
    <div class="document-meta">
      <div class="field"><label>Category</label><select data-doc-category="${doc.id}">${CATEGORIES.map(x=>`<option ${x===doc.category?"selected":""}>${x}</option>`).join("")}</select></div>
      <div class="field"><label>Sensitivity</label><select data-doc-sensitivity="${doc.id}">${SENSITIVITY.map(x=>`<option ${x===doc.sensitivity?"selected":""}>${x}</option>`).join("")}</select></div>
    </div>
    <div class="recipient-picker compact"><label>Intended recipients</label>${STAKEHOLDERS.map(x=>`<label class="recipient-option"><input type="checkbox" data-doc-recipient="${doc.id}" value="${x}" ${doc.recipients.includes(x)?"checked":""}/> <span>${x}</span></label>`).join("")}</div>
    <div class="document-actions"><a class="ghost button-link" href="/api/documents/${doc.id}/download">Download</a><button class="danger-button" data-doc-delete="${doc.id}">Delete</button></div>
  </div>`;
}
async function uploadDocument(event){
  event.preventDefault();
  const file=document.querySelector("#lockerFile").files[0];
  const error=document.querySelector("#uploadError");
  const button=document.querySelector("#uploadButton");
  error.textContent="";
  if(!file){error.textContent="Choose a document first.";return}
  if(file.size>25*1024*1024){error.textContent="Each document must be 25 MB or smaller.";return}
  button.disabled=true;button.textContent="Adding securely…";
  try{
    const data=await new Promise((resolve,reject)=>{
      const reader=new FileReader();
      reader.onload=()=>resolve(String(reader.result).split(",")[1]);
      reader.onerror=reject;
      reader.readAsDataURL(file);
    });
    const response=await fetch("/api/documents",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({
      filename:file.name,contentType:file.type||"application/octet-stream",data,
      category:document.querySelector("#uploadCategory").value,
      sensitivity:document.querySelector("#uploadSensitivity").value,
      recipients:[...document.querySelectorAll('[name="uploadRecipient"]:checked')].map(x=>x.value)
    })});
    const result=await response.json();
    if(!response.ok) throw new Error(result.error||"Upload failed");
    documents.unshift(result);toast("Document added to private locker");render(false);
  }catch(err){error.textContent=err.message;button.disabled=false;button.textContent="Add to private locker"}
}
async function deleteDocument(id){
  if(!confirm("Delete this document from the private locker?")) return;
  const response=await fetch(`/api/documents/${id}`,{method:"DELETE"});
  if(!response.ok){toast("Could not delete document");return}
  documents=documents.filter(x=>x.id!==id);toast("Document deleted");render(false);
}
async function updateDocumentMeta(id){
  const doc=documents.find(x=>x.id===id);if(!doc)return;
  const category=document.querySelector(`[data-doc-category="${id}"]`)?.value||doc.category;
  const sensitivity=document.querySelector(`[data-doc-sensitivity="${id}"]`)?.value||doc.sensitivity;
  const recipients=[...document.querySelectorAll(`[data-doc-recipient="${id}"]:checked`)].map(x=>x.value);
  const response=await fetch(`/api/documents/${id}`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({category,sensitivity,recipients})});
  if(response.ok){Object.assign(doc,{category,sensitivity,recipients});toast("Document access updated")}
  else toast("Could not update document");
}

function handoffView(){
  const ss=scenarios(), selected=ss.find(s=>s.id===state.selectedScenario);
  const tasks=[
    ["Confirm owner objectives","Review destination, must-haves and avoidances","Owner"],
    ["Validate business value","Independent valuation and fairness work","Valuation professional"],
    ["Test financing capacity","Confirm debt service and capital sources","Lender / advisor"],
    ["Evaluate structure","Review ESOP, bridge and direct-purchase alternatives","ESOP counsel"],
    ["Analyze tax treatment","Model seller and company-level consequences","CPA / tax counsel"],
    ["Assess employee/governance readiness","Leadership, communications and fiduciary plan","ESOP advisor"]
  ];
  return `<div class="hero-copy"><div class="layer-badges"><span class="owner-layer">Objectives captured</span><span class="platform-layer">Scenario prepared</span><span class="pro-layer">Determination needed</span></div>
  <h2>Turn exploration into an advisor-ready conversation.</h2><p>Your next best action is not to “choose the winner.” It is to approve a controlled package of objectives, assumptions, documents and open questions for the right professionals.</p></div>
  <div class="grid-2">
    <div class="card"><div class="eyebrow">CURRENT WORKING DIRECTION</div><h3>${selected?selected.name:"Select a scenario to carry forward"}</h3><p class="helper">${selected?selected.why:"You can still export the objective brief before selecting a path."}</p>
      ${selected?`<div class="metric-row" style="grid-template-columns:1fr 1fr;margin-top:20px"><div class="metric"><span>Cash at close</span><strong>${money(selected.cash)}</strong></div><div class="metric"><span>Seller note</span><strong>${money(selected.note)}</strong></div></div>`:""}
    </div>
    <div class="card"><div class="eyebrow">QUESTIONS FOR YOUR ADVISORS</div><ul class="helper" style="line-height:2;margin:0;padding-left:18px">
      <li>What independent valuation range is supportable?</li><li>How much financing can the company responsibly carry?</li><li>Which structure can deliver 100% employee ownership?</li><li>What tax, fiduciary and regulatory requirements apply?</li><li>What must be true for employees after closing?</li>
    </ul></div>
  </div>
  <div class="section-title"><h3>Professional review plan</h3><p>Every determination stays attributed to the professional who made it.</p></div>
  <div class="card checklist">${tasks.map((x,i)=>`<div class="check-item"><b>${i+1}</b><div style="flex:1"><strong>${x[0]}</strong><span>${x[1]}</span></div><span>${x[2]}</span></div>`).join("")}</div>
  <div class="section-title"><h3>Document package readiness</h3></div>
  <div class="card package-summary"><div><span>Checklist ready</span><strong>${LOCKER_ITEMS.filter(x=>lockerStatus(x)==="Ready").length}/${LOCKER_ITEMS.length}</strong></div><div><span>Documents stored</span><strong>${documents.length}</strong></div><div><span>Recipient groups assigned</span><strong>${new Set(documents.flatMap(x=>x.recipients)).size}</strong></div><button class="ghost" data-nav="5">Review document locker</button></div>
  <div class="section-title"><h3>What your brief contains</h3></div>
  <div class="grid-3">
    <div class="card"><div class="eyebrow">PART I</div><h3>Owner objective</h3><p class="helper">Goals, priorities, nonnegotiables, timing and outcomes to avoid.</p></div>
    <div class="card"><div class="eyebrow">PART II</div><h3>Platform scenarios</h3><p class="helper">Structures explored, stated assumptions, modeled outputs and trade-offs.</p></div>
    <div class="card"><div class="eyebrow">PART III</div><h3>Controlled document package</h3><p class="helper">Classified source files with intended recipients and readiness status.</p></div>
    <div class="card"><div class="eyebrow">PART IV</div><h3>Professional determinations</h3><p class="helper">Attributed legal, tax, valuation, lending and fiduciary conclusions.</p></div>
  </div>
  <div class="callout" style="margin-top:22px"><strong>Your plan remains editable.</strong>Changing an objective should create a new scenario—not erase the history of what you explored or why.</div>`;
}

$("#nextBtn").onclick=()=>state.currentStep===STEPS.length-1?save(true):completeAndNext();
$("#backBtn").onclick=()=>{if(state.currentStep){state.currentStep--;save();render();window.scrollTo({top:0,behavior:"smooth"})}};
$("#principlesBtn").onclick=()=>$("#principlesModal").classList.remove("hidden");
$("#closeModal").onclick=()=>$("#principlesModal").classList.add("hidden");
$("#principlesModal").onclick=e=>{if(e.target.id==="principlesModal")e.currentTarget.classList.add("hidden")};
$("#printBtn").onclick=()=>{const old=state.currentStep;state.currentStep=4;render();setTimeout(()=>{window.print();state.currentStep=old;render()},100)};
load();