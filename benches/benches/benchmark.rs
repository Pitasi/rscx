use criterion::{criterion_group, criterion_main, Criterion};
use futures::StreamExt;
use leptos::IntoView;
use maud::html;
use rscx::props;

fn criterion_benchmark(c: &mut Criterion) {
    let async_rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    let mut g = c.benchmark_group("many_attrs");
    g.bench_function("maud_many_attrs", |b| b.iter(|| maud_many_attrs()));
    g.bench_function("html_node_many_attrs", |b| {
        b.iter(|| html_node_many_attrs())
    });
    g.bench_function("leptos_many_attrs", |b| b.iter(|| leptos_many_attrs()));
    g.bench_function("rscx_many_attrs", |b| b.iter(|| rscx_many_attrs()));
    g.finish();

    let mut g = c.benchmark_group("small_fragment");
    g.bench_function("maud_small_fragment", |b| b.iter(|| maud_small_fragment()));
    g.bench_function("leptos_small_fragment", |b| {
        b.iter(|| leptos_small_fragment())
    });
    g.bench_function("rscx_small_fragment", |b| {
        b.to_async(&async_rt).iter(|| rscx_small_fragment())
    });
    g.finish();

    let mut g = c.benchmark_group("async_list");
    g.bench_function("maud_async_list", |b| {
        b.to_async(&async_rt).iter(|| maud_async_list_app())
    });
    g.bench_function("leptos_async_list", |b| {
        b.to_async(&async_rt).iter(|| leptos_async_list_app())
    });
    g.bench_function("rscx_async_list", |b| {
        b.to_async(&async_rt).iter(|| rscx_async_list_app())
    });
    g.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn maud_many_attrs() -> String {
    html! {
        h1 attr1="1" attr2="2" attr3="3" attr4="4" attr5="5" attr6="6" attr7="7" attr8="8" attr9="9" attr10="10" attr11="11" attr12="12" attr13="13" attr14="14" attr15="15" attr16="16" attr17="17" attr18="18" attr19="19" attr20="20" attr21="21" attr22="22" attr23="23" attr24="24" attr25="25" attr26="26" attr27="27" attr28="28" attr29="29" attr30="30" attr31="31" attr32="32" attr33="33" attr34="34" attr35="35" attr36="36" attr37="37" attr38="38" attr39="39" attr40="40" attr41="41" attr42="42" attr43="43" attr44="44" attr45="45" attr46="46" attr47="47" attr48="48" attr49="49" attr50="50" attr51="51" attr52="52" attr53="53" attr54="54" attr55="55" attr56="56" attr57="57" attr58="58" attr59="59" attr60="60" attr61="61" attr62="62" attr63="63" attr64="64" attr65="65" attr66="66" attr67="67" attr68="68" attr69="69" attr70="70" attr71="71" attr72="72" attr73="73" attr74="74" attr75="75" attr76="76" attr77="77" attr78="78" attr79="79" attr80="80" attr81="81" attr82="82" attr83="83" attr84="84" attr85="85" attr86="86" attr87="87" attr88="88" attr89="89" attr90="90" attr91="91" attr92="92" attr93="93" attr94="94" attr95="95" attr96="96" attr97="97" attr98="98" attr99="99" attr100="100" attr101="101" attr102="102" attr103="103" attr104="104" attr105="105" attr106="106" attr107="107" attr108="108" attr109="109" attr110="110" attr111="111" attr112="112" attr113="113" attr114="114" attr115="115" attr116="116" attr117="117" attr118="118" attr119="119" attr120="120" attr121="121" attr122="122" attr123="123" attr124="124" attr125="125" attr126="126" attr127="127" attr128="128" attr129="129" attr130="130" attr131="131" attr132="132" attr133="133" attr134="134" attr135="135" attr136="136" attr137="137" attr138="138" attr139="139" attr140="140" attr141="141" attr142="142" attr143="143" attr144="144" attr145="145" attr146="146" attr147="147" attr148="148" attr149="149" attr150="150" attr151="151" attr152="152" attr153="153" attr154="154" attr155="155" attr156="156" attr157="157" attr158="158" attr159="159" attr160="160" attr161="161" attr162="162" attr163="163" attr164="164" attr165="165" attr166="166" attr167="167" attr168="168" attr169="169" attr170="170" attr171="171" attr172="172" attr173="173" attr174="174" attr175="175" attr176="176" attr177="177" attr178="178" attr179="179" attr180="180" attr181="181" attr182="182" attr183="183" attr184="184" attr185="185" attr186="186" attr187="187" attr188="188" attr189="189" attr190="190" attr191="191" attr192="192" attr193="193" attr194="194" attr195="195" attr196="196" attr197="197" attr198="198" attr199="199" attr200="200" attr201="201" attr202="202" attr203="203" attr204="204" attr205="205" attr206="206" attr207="207" attr208="208" attr209="209" attr210="210" attr211="211" attr212="212" attr213="213" attr214="214" attr215="215" attr216="216" attr217="217" attr218="218" attr219="219" attr220="220" attr221="221" attr222="222" attr223="223" attr224="224" attr225="225" attr226="226" attr227="227" attr228="228" attr229="229" attr230="230" attr231="231" attr232="232" attr233="233" attr234="234" attr235="235" attr236="236" attr237="237" attr238="238" attr239="239" attr240="240" attr241="241" attr242="242" attr243="243" attr244="244" attr245="245" attr246="246" attr247="247" attr248="248" attr249="249" attr250="250" attr251="251" attr252="252" attr253="253" attr254="254" attr255="255" attr256="256" attr257="257" attr258="258" attr259="259" attr260="260" attr261="261" attr262="262" attr263="263" attr264="264" attr265="265" attr266="266" attr267="267" attr268="268" attr269="269" attr270="270" attr271="271" attr272="272" attr273="273" attr274="274" attr275="275" attr276="276" attr277="277" attr278="278" attr279="279" attr280="280" attr281="281" attr282="282" attr283="283" attr284="284" attr285="285" attr286="286" attr287="287" attr288="288" attr289="289" attr290="290" attr291="291" attr292="292" attr293="293" attr294="294" attr295="295" attr296="296" attr297="297" attr298="298" attr299="299" attr300="300" attr301="301" attr302="302" attr303="303" attr304="304" attr305="305" attr306="306" attr307="307" attr308="308" attr309="309" attr310="310" attr311="311" attr312="312" attr313="313" attr314="314" attr315="315" attr316="316" attr317="317" attr318="318" attr319="319" attr320="320" attr321="321" attr322="322" attr323="323" attr324="324" attr325="325" attr326="326" attr327="327" attr328="328" attr329="329" attr330="330" attr331="331" attr332="332" attr333="333" attr334="334" attr335="335" attr336="336" attr337="337" attr338="338" attr339="339" attr340="340" attr341="341" attr342="342" attr343="343" attr344="344" attr345="345" attr346="346" attr347="347" attr348="348" attr349="349" attr350="350" attr351="351" attr352="352" attr353="353" attr354="354" attr355="355" attr356="356" attr357="357" attr358="358" attr359="359" attr360="360" attr361="361" attr362="362" attr363="363" attr364="364" attr365="365" attr366="366" attr367="367" attr368="368" attr369="369" attr370="370" attr371="371" attr372="372" attr373="373" attr374="374" attr375="375" attr376="376" attr377="377" attr378="378" attr379="379" attr380="380" attr381="381" attr382="382" attr383="383" attr384="384" attr385="385" attr386="386" attr387="387" attr388="388" attr389="389" attr390="390" attr391="391" attr392="392" attr393="393" attr394="394" attr395="395" attr396="396" attr397="397" attr398="398" attr399="399" attr400="400" attr401="401" attr402="402" attr403="403" attr404="404" attr405="405" attr406="406" attr407="407" attr408="408" attr409="409" attr410="410" attr411="411" attr412="412" attr413="413" attr414="414" attr415="415" attr416="416" attr417="417" attr418="418" attr419="419" attr420="420" attr421="421" attr422="422" attr423="423" attr424="424" attr425="425" attr426="426" attr427="427" attr428="428" attr429="429" attr430="430" attr431="431" attr432="432" attr433="433" attr434="434" attr435="435" attr436="436" attr437="437" attr438="438" attr439="439" attr440="440" attr441="441" attr442="442" attr443="443" attr444="444" attr445="445" attr446="446" attr447="447" attr448="448" attr449="449" attr450="450" attr451="451" attr452="452" attr453="453" attr454="454" attr455="455" attr456="456" attr457="457" attr458="458" attr459="459" attr460="460" attr461="461" attr462="462" attr463="463" attr464="464" attr465="465" attr466="466" attr467="467" attr468="468" attr469="469" attr470="470" attr471="471" attr472="472" attr473="473" attr474="474" attr475="475" attr476="476" attr477="477" attr478="478" attr479="479" attr480="480" attr481="481" attr482="482" attr483="483" attr484="484" attr485="485" attr486="486" attr487="487" attr488="488" attr489="489" attr490="490" attr491="491" attr492="492" attr493="493" attr494="494" attr495="495" attr496="496" attr497="497" attr498="498" attr499="499" attr500="500" attr501="501" attr502="502" attr503="503" attr504="504" attr505="505" attr506="506" attr507="507" attr508="508" attr509="509" attr510="510" attr511="511" attr512="512" attr513="513" attr514="514" attr515="515" attr516="516" attr517="517" attr518="518" attr519="519" attr520="520" attr521="521" attr522="522" attr523="523" attr524="524" attr525="525" attr526="526" attr527="527" attr528="528" attr529="529" attr530="530" attr531="531" attr532="532" attr533="533" attr534="534" attr535="535" attr536="536" attr537="537" attr538="538" attr539="539" attr540="540" attr541="541" attr542="542" attr543="543" attr544="544" attr545="545" attr546="546" attr547="547" attr548="548" attr549="549" attr550="550" attr551="551" attr552="552" attr553="553" attr554="554" attr555="555" attr556="556" attr557="557" attr558="558" attr559="559" attr560="560" attr561="561" attr562="562" attr563="563" attr564="564" attr565="565" attr566="566" attr567="567" attr568="568" attr569="569" attr570="570" attr571="571" attr572="572" attr573="573" attr574="574" attr575="575" attr576="576" attr577="577" attr578="578" attr579="579" attr580="580" attr581="581" attr582="582" attr583="583" attr584="584" attr585="585" attr586="586" attr587="587" attr588="588" attr589="589" attr590="590" attr591="591" attr592="592" attr593="593" attr594="594" attr595="595" attr596="596" attr597="597" attr598="598" attr599="599" attr600="600" attr601="601" attr602="602" attr603="603" attr604="604" attr605="605" attr606="606" attr607="607" attr608="608" attr609="609" attr610="610" attr611="611" attr612="612" attr613="613" attr614="614" attr615="615" attr616="616" attr617="617" attr618="618" attr619="619" attr620="620" attr621="621" attr622="622" attr623="623" attr624="624" attr625="625" attr626="626" attr627="627" attr628="628" attr629="629" attr630="630" attr631="631" attr632="632" attr633="633" attr634="634" attr635="635" attr636="636" attr637="637" attr638="638" attr639="639" attr640="640" attr641="641" attr642="642" attr643="643" attr644="644" attr645="645" attr646="646" attr647="647" attr648="648" attr649="649" attr650="650" attr651="651" attr652="652" attr653="653" attr654="654" attr655="655" attr656="656" attr657="657" attr658="658" attr659="659" attr660="660" attr661="661" attr662="662" attr663="663" attr664="664" attr665="665" attr666="666" attr667="667" attr668="668" attr669="669" attr670="670" attr671="671" attr672="672" attr673="673" attr674="674" attr675="675" attr676="676" attr677="677" attr678="678" attr679="679" attr680="680" attr681="681" attr682="682" attr683="683" attr684="684" attr685="685" attr686="686" attr687="687" attr688="688" attr689="689" attr690="690" attr691="691" attr692="692" attr693="693" attr694="694" attr695="695" attr696="696" attr697="697" attr698="698" attr699="699" attr700="700" attr701="701" attr702="702" attr703="703" attr704="704" attr705="705" attr706="706" attr707="707" attr708="708" attr709="709" attr710="710" attr711="711" attr712="712" attr713="713" attr714="714" attr715="715" attr716="716" attr717="717" attr718="718" attr719="719" attr720="720" attr721="721" attr722="722" attr723="723" attr724="724" attr725="725" attr726="726" attr727="727" attr728="728" attr729="729" attr730="730" attr731="731" attr732="732" attr733="733" attr734="734" attr735="735" attr736="736" attr737="737" attr738="738" attr739="739" attr740="740" attr741="741" attr742="742" attr743="743" attr744="744" attr745="745" attr746="746" attr747="747" attr748="748" attr749="749" attr750="750" attr751="751" attr752="752" attr753="753" attr754="754" attr755="755" attr756="756" attr757="757" attr758="758" attr759="759" attr760="760" attr761="761" attr762="762" attr763="763" attr764="764" attr765="765" attr766="766" attr767="767" attr768="768" attr769="769" attr770="770" attr771="771" attr772="772" attr773="773" attr774="774" attr775="775" attr776="776" attr777="777" attr778="778" attr779="779" attr780="780" attr781="781" attr782="782" attr783="783" attr784="784" attr785="785" attr786="786" attr787="787" attr788="788" attr789="789" attr790="790" attr791="791" attr792="792" attr793="793" attr794="794" attr795="795" attr796="796" attr797="797" attr798="798" attr799="799" attr800="800" attr801="801" attr802="802" attr803="803" attr804="804" attr805="805" attr806="806" attr807="807" attr808="808" attr809="809" attr810="810" attr811="811" attr812="812" attr813="813" attr814="814" attr815="815" attr816="816" attr817="817" attr818="818" attr819="819" attr820="820" attr821="821" attr822="822" attr823="823" attr824="824" attr825="825" attr826="826" attr827="827" attr828="828" attr829="829" attr830="830" attr831="831" attr832="832" attr833="833" attr834="834" attr835="835" attr836="836" attr837="837" attr838="838" attr839="839" attr840="840" attr841="841" attr842="842" attr843="843" attr844="844" attr845="845" attr846="846" attr847="847" attr848="848" attr849="849" attr850="850" attr851="851" attr852="852" attr853="853" attr854="854" attr855="855" attr856="856" attr857="857" attr858="858" attr859="859" attr860="860" attr861="861" attr862="862" attr863="863" attr864="864" attr865="865" attr866="866" attr867="867" attr868="868" attr869="869" attr870="870" attr871="871" attr872="872" attr873="873" attr874="874" attr875="875" attr876="876" attr877="877" attr878="878" attr879="879" attr880="880" attr881="881" attr882="882" attr883="883" attr884="884" attr885="885" attr886="886" attr887="887" attr888="888" attr889="889" attr890="890" attr891="891" attr892="892" attr893="893" attr894="894" attr895="895" attr896="896" attr897="897" attr898="898" attr899="899" attr900="900" attr901="901" attr902="902" attr903="903" attr904="904" attr905="905" attr906="906" attr907="907" attr908="908" attr909="909" attr910="910" attr911="911" attr912="912" attr913="913" attr914="914" attr915="915" attr916="916" attr917="917" attr918="918" attr919="919" attr920="920" attr921="921" attr922="922" attr923="923" attr924="924" attr925="925" attr926="926" attr927="927" attr928="928" attr929="929" attr930="930" attr931="931" attr932="932" attr933="933" attr934="934" attr935="935" attr936="936" attr937="937" attr938="938" attr939="939" attr940="940" attr941="941" attr942="942" attr943="943" attr944="944" attr945="945" attr946="946" attr947="947" attr948="948" attr949="949" attr950="950" attr951="951" attr952="952" attr953="953" attr954="954" attr955="955" attr956="956" attr957="957" attr958="958" attr959="959" attr960="960" attr961="961" attr962="962" attr963="963" attr964="964" attr965="965" attr966="966" attr967="967" attr968="968" attr969="969" attr970="970" attr971="971" attr972="972" attr973="973" attr974="974" attr975="975" attr976="976" attr977="977" attr978="978" attr979="979" attr980="980" attr981="981" attr982="982" attr983="983" attr984="984" attr985="985" attr986="986" attr987="987" attr988="988" attr989="989" attr990="990" attr991="991" attr992="992" attr993="993" attr994="994" attr995="995" attr996="996" attr997="997" attr998="998" attr999="999" attr1000="1000"
        { "nice" }
    }.0
}

fn rscx_many_attrs() -> String {
    rscx::html! {
        <h1 attr1="1" attr2="2" attr3="3" attr4="4" attr5="5" attr6="6" attr7="7" attr8="8" attr9="9" attr10="10" attr11="11" attr12="12" attr13="13" attr14="14" attr15="15" attr16="16" attr17="17" attr18="18" attr19="19" attr20="20" attr21="21" attr22="22" attr23="23" attr24="24" attr25="25" attr26="26" attr27="27" attr28="28" attr29="29" attr30="30" attr31="31" attr32="32" attr33="33" attr34="34" attr35="35" attr36="36" attr37="37" attr38="38" attr39="39" attr40="40" attr41="41" attr42="42" attr43="43" attr44="44" attr45="45" attr46="46" attr47="47" attr48="48" attr49="49" attr50="50" attr51="51" attr52="52" attr53="53" attr54="54" attr55="55" attr56="56" attr57="57" attr58="58" attr59="59" attr60="60" attr61="61" attr62="62" attr63="63" attr64="64" attr65="65" attr66="66" attr67="67" attr68="68" attr69="69" attr70="70" attr71="71" attr72="72" attr73="73" attr74="74" attr75="75" attr76="76" attr77="77" attr78="78" attr79="79" attr80="80" attr81="81" attr82="82" attr83="83" attr84="84" attr85="85" attr86="86" attr87="87" attr88="88" attr89="89" attr90="90" attr91="91" attr92="92" attr93="93" attr94="94" attr95="95" attr96="96" attr97="97" attr98="98" attr99="99" attr100="100" attr101="101" attr102="102" attr103="103" attr104="104" attr105="105" attr106="106" attr107="107" attr108="108" attr109="109" attr110="110" attr111="111" attr112="112" attr113="113" attr114="114" attr115="115" attr116="116" attr117="117" attr118="118" attr119="119" attr120="120" attr121="121" attr122="122" attr123="123" attr124="124" attr125="125" attr126="126" attr127="127" attr128="128" attr129="129" attr130="130" attr131="131" attr132="132" attr133="133" attr134="134" attr135="135" attr136="136" attr137="137" attr138="138" attr139="139" attr140="140" attr141="141" attr142="142" attr143="143" attr144="144" attr145="145" attr146="146" attr147="147" attr148="148" attr149="149" attr150="150" attr151="151" attr152="152" attr153="153" attr154="154" attr155="155" attr156="156" attr157="157" attr158="158" attr159="159" attr160="160" attr161="161" attr162="162" attr163="163" attr164="164" attr165="165" attr166="166" attr167="167" attr168="168" attr169="169" attr170="170" attr171="171" attr172="172" attr173="173" attr174="174" attr175="175" attr176="176" attr177="177" attr178="178" attr179="179" attr180="180" attr181="181" attr182="182" attr183="183" attr184="184" attr185="185" attr186="186" attr187="187" attr188="188" attr189="189" attr190="190" attr191="191" attr192="192" attr193="193" attr194="194" attr195="195" attr196="196" attr197="197" attr198="198" attr199="199" attr200="200" attr201="201" attr202="202" attr203="203" attr204="204" attr205="205" attr206="206" attr207="207" attr208="208" attr209="209" attr210="210" attr211="211" attr212="212" attr213="213" attr214="214" attr215="215" attr216="216" attr217="217" attr218="218" attr219="219" attr220="220" attr221="221" attr222="222" attr223="223" attr224="224" attr225="225" attr226="226" attr227="227" attr228="228" attr229="229" attr230="230" attr231="231" attr232="232" attr233="233" attr234="234" attr235="235" attr236="236" attr237="237" attr238="238" attr239="239" attr240="240" attr241="241" attr242="242" attr243="243" attr244="244" attr245="245" attr246="246" attr247="247" attr248="248" attr249="249" attr250="250" attr251="251" attr252="252" attr253="253" attr254="254" attr255="255" attr256="256" attr257="257" attr258="258" attr259="259" attr260="260" attr261="261" attr262="262" attr263="263" attr264="264" attr265="265" attr266="266" attr267="267" attr268="268" attr269="269" attr270="270" attr271="271" attr272="272" attr273="273" attr274="274" attr275="275" attr276="276" attr277="277" attr278="278" attr279="279" attr280="280" attr281="281" attr282="282" attr283="283" attr284="284" attr285="285" attr286="286" attr287="287" attr288="288" attr289="289" attr290="290" attr291="291" attr292="292" attr293="293" attr294="294" attr295="295" attr296="296" attr297="297" attr298="298" attr299="299" attr300="300" attr301="301" attr302="302" attr303="303" attr304="304" attr305="305" attr306="306" attr307="307" attr308="308" attr309="309" attr310="310" attr311="311" attr312="312" attr313="313" attr314="314" attr315="315" attr316="316" attr317="317" attr318="318" attr319="319" attr320="320" attr321="321" attr322="322" attr323="323" attr324="324" attr325="325" attr326="326" attr327="327" attr328="328" attr329="329" attr330="330" attr331="331" attr332="332" attr333="333" attr334="334" attr335="335" attr336="336" attr337="337" attr338="338" attr339="339" attr340="340" attr341="341" attr342="342" attr343="343" attr344="344" attr345="345" attr346="346" attr347="347" attr348="348" attr349="349" attr350="350" attr351="351" attr352="352" attr353="353" attr354="354" attr355="355" attr356="356" attr357="357" attr358="358" attr359="359" attr360="360" attr361="361" attr362="362" attr363="363" attr364="364" attr365="365" attr366="366" attr367="367" attr368="368" attr369="369" attr370="370" attr371="371" attr372="372" attr373="373" attr374="374" attr375="375" attr376="376" attr377="377" attr378="378" attr379="379" attr380="380" attr381="381" attr382="382" attr383="383" attr384="384" attr385="385" attr386="386" attr387="387" attr388="388" attr389="389" attr390="390" attr391="391" attr392="392" attr393="393" attr394="394" attr395="395" attr396="396" attr397="397" attr398="398" attr399="399" attr400="400" attr401="401" attr402="402" attr403="403" attr404="404" attr405="405" attr406="406" attr407="407" attr408="408" attr409="409" attr410="410" attr411="411" attr412="412" attr413="413" attr414="414" attr415="415" attr416="416" attr417="417" attr418="418" attr419="419" attr420="420" attr421="421" attr422="422" attr423="423" attr424="424" attr425="425" attr426="426" attr427="427" attr428="428" attr429="429" attr430="430" attr431="431" attr432="432" attr433="433" attr434="434" attr435="435" attr436="436" attr437="437" attr438="438" attr439="439" attr440="440" attr441="441" attr442="442" attr443="443" attr444="444" attr445="445" attr446="446" attr447="447" attr448="448" attr449="449" attr450="450" attr451="451" attr452="452" attr453="453" attr454="454" attr455="455" attr456="456" attr457="457" attr458="458" attr459="459" attr460="460" attr461="461" attr462="462" attr463="463" attr464="464" attr465="465" attr466="466" attr467="467" attr468="468" attr469="469" attr470="470" attr471="471" attr472="472" attr473="473" attr474="474" attr475="475" attr476="476" attr477="477" attr478="478" attr479="479" attr480="480" attr481="481" attr482="482" attr483="483" attr484="484" attr485="485" attr486="486" attr487="487" attr488="488" attr489="489" attr490="490" attr491="491" attr492="492" attr493="493" attr494="494" attr495="495" attr496="496" attr497="497" attr498="498" attr499="499" attr500="500" attr501="501" attr502="502" attr503="503" attr504="504" attr505="505" attr506="506" attr507="507" attr508="508" attr509="509" attr510="510" attr511="511" attr512="512" attr513="513" attr514="514" attr515="515" attr516="516" attr517="517" attr518="518" attr519="519" attr520="520" attr521="521" attr522="522" attr523="523" attr524="524" attr525="525" attr526="526" attr527="527" attr528="528" attr529="529" attr530="530" attr531="531" attr532="532" attr533="533" attr534="534" attr535="535" attr536="536" attr537="537" attr538="538" attr539="539" attr540="540" attr541="541" attr542="542" attr543="543" attr544="544" attr545="545" attr546="546" attr547="547" attr548="548" attr549="549" attr550="550" attr551="551" attr552="552" attr553="553" attr554="554" attr555="555" attr556="556" attr557="557" attr558="558" attr559="559" attr560="560" attr561="561" attr562="562" attr563="563" attr564="564" attr565="565" attr566="566" attr567="567" attr568="568" attr569="569" attr570="570" attr571="571" attr572="572" attr573="573" attr574="574" attr575="575" attr576="576" attr577="577" attr578="578" attr579="579" attr580="580" attr581="581" attr582="582" attr583="583" attr584="584" attr585="585" attr586="586" attr587="587" attr588="588" attr589="589" attr590="590" attr591="591" attr592="592" attr593="593" attr594="594" attr595="595" attr596="596" attr597="597" attr598="598" attr599="599" attr600="600" attr601="601" attr602="602" attr603="603" attr604="604" attr605="605" attr606="606" attr607="607" attr608="608" attr609="609" attr610="610" attr611="611" attr612="612" attr613="613" attr614="614" attr615="615" attr616="616" attr617="617" attr618="618" attr619="619" attr620="620" attr621="621" attr622="622" attr623="623" attr624="624" attr625="625" attr626="626" attr627="627" attr628="628" attr629="629" attr630="630" attr631="631" attr632="632" attr633="633" attr634="634" attr635="635" attr636="636" attr637="637" attr638="638" attr639="639" attr640="640" attr641="641" attr642="642" attr643="643" attr644="644" attr645="645" attr646="646" attr647="647" attr648="648" attr649="649" attr650="650" attr651="651" attr652="652" attr653="653" attr654="654" attr655="655" attr656="656" attr657="657" attr658="658" attr659="659" attr660="660" attr661="661" attr662="662" attr663="663" attr664="664" attr665="665" attr666="666" attr667="667" attr668="668" attr669="669" attr670="670" attr671="671" attr672="672" attr673="673" attr674="674" attr675="675" attr676="676" attr677="677" attr678="678" attr679="679" attr680="680" attr681="681" attr682="682" attr683="683" attr684="684" attr685="685" attr686="686" attr687="687" attr688="688" attr689="689" attr690="690" attr691="691" attr692="692" attr693="693" attr694="694" attr695="695" attr696="696" attr697="697" attr698="698" attr699="699" attr700="700" attr701="701" attr702="702" attr703="703" attr704="704" attr705="705" attr706="706" attr707="707" attr708="708" attr709="709" attr710="710" attr711="711" attr712="712" attr713="713" attr714="714" attr715="715" attr716="716" attr717="717" attr718="718" attr719="719" attr720="720" attr721="721" attr722="722" attr723="723" attr724="724" attr725="725" attr726="726" attr727="727" attr728="728" attr729="729" attr730="730" attr731="731" attr732="732" attr733="733" attr734="734" attr735="735" attr736="736" attr737="737" attr738="738" attr739="739" attr740="740" attr741="741" attr742="742" attr743="743" attr744="744" attr745="745" attr746="746" attr747="747" attr748="748" attr749="749" attr750="750" attr751="751" attr752="752" attr753="753" attr754="754" attr755="755" attr756="756" attr757="757" attr758="758" attr759="759" attr760="760" attr761="761" attr762="762" attr763="763" attr764="764" attr765="765" attr766="766" attr767="767" attr768="768" attr769="769" attr770="770" attr771="771" attr772="772" attr773="773" attr774="774" attr775="775" attr776="776" attr777="777" attr778="778" attr779="779" attr780="780" attr781="781" attr782="782" attr783="783" attr784="784" attr785="785" attr786="786" attr787="787" attr788="788" attr789="789" attr790="790" attr791="791" attr792="792" attr793="793" attr794="794" attr795="795" attr796="796" attr797="797" attr798="798" attr799="799" attr800="800" attr801="801" attr802="802" attr803="803" attr804="804" attr805="805" attr806="806" attr807="807" attr808="808" attr809="809" attr810="810" attr811="811" attr812="812" attr813="813" attr814="814" attr815="815" attr816="816" attr817="817" attr818="818" attr819="819" attr820="820" attr821="821" attr822="822" attr823="823" attr824="824" attr825="825" attr826="826" attr827="827" attr828="828" attr829="829" attr830="830" attr831="831" attr832="832" attr833="833" attr834="834" attr835="835" attr836="836" attr837="837" attr838="838" attr839="839" attr840="840" attr841="841" attr842="842" attr843="843" attr844="844" attr845="845" attr846="846" attr847="847" attr848="848" attr849="849" attr850="850" attr851="851" attr852="852" attr853="853" attr854="854" attr855="855" attr856="856" attr857="857" attr858="858" attr859="859" attr860="860" attr861="861" attr862="862" attr863="863" attr864="864" attr865="865" attr866="866" attr867="867" attr868="868" attr869="869" attr870="870" attr871="871" attr872="872" attr873="873" attr874="874" attr875="875" attr876="876" attr877="877" attr878="878" attr879="879" attr880="880" attr881="881" attr882="882" attr883="883" attr884="884" attr885="885" attr886="886" attr887="887" attr888="888" attr889="889" attr890="890" attr891="891" attr892="892" attr893="893" attr894="894" attr895="895" attr896="896" attr897="897" attr898="898" attr899="899" attr900="900" attr901="901" attr902="902" attr903="903" attr904="904" attr905="905" attr906="906" attr907="907" attr908="908" attr909="909" attr910="910" attr911="911" attr912="912" attr913="913" attr914="914" attr915="915" attr916="916" attr917="917" attr918="918" attr919="919" attr920="920" attr921="921" attr922="922" attr923="923" attr924="924" attr925="925" attr926="926" attr927="927" attr928="928" attr929="929" attr930="930" attr931="931" attr932="932" attr933="933" attr934="934" attr935="935" attr936="936" attr937="937" attr938="938" attr939="939" attr940="940" attr941="941" attr942="942" attr943="943" attr944="944" attr945="945" attr946="946" attr947="947" attr948="948" attr949="949" attr950="950" attr951="951" attr952="952" attr953="953" attr954="954" attr955="955" attr956="956" attr957="957" attr958="958" attr959="959" attr960="960" attr961="961" attr962="962" attr963="963" attr964="964" attr965="965" attr966="966" attr967="967" attr968="968" attr969="969" attr970="970" attr971="971" attr972="972" attr973="973" attr974="974" attr975="975" attr976="976" attr977="977" attr978="978" attr979="979" attr980="980" attr981="981" attr982="982" attr983="983" attr984="984" attr985="985" attr986="986" attr987="987" attr988="988" attr989="989" attr990="990" attr991="991" attr992="992" attr993="993" attr994="994" attr995="995" attr996="996" attr997="997" attr998="998" attr999="999" attr1000="1000">
            "nice"
        </h1>
    }
}

fn leptos_many_attrs() -> leptos::Oco<'static, str> {
    leptos::ssr::render_to_string(|| {
        leptos::view! {
            <h1 attr1="1" attr2="2" attr3="3" attr4="4" attr5="5" attr6="6" attr7="7" attr8="8" attr9="9" attr10="10" attr11="11" attr12="12" attr13="13" attr14="14" attr15="15" attr16="16" attr17="17" attr18="18" attr19="19" attr20="20" attr21="21" attr22="22" attr23="23" attr24="24" attr25="25" attr26="26" attr27="27" attr28="28" attr29="29" attr30="30" attr31="31" attr32="32" attr33="33" attr34="34" attr35="35" attr36="36" attr37="37" attr38="38" attr39="39" attr40="40" attr41="41" attr42="42" attr43="43" attr44="44" attr45="45" attr46="46" attr47="47" attr48="48" attr49="49" attr50="50" attr51="51" attr52="52" attr53="53" attr54="54" attr55="55" attr56="56" attr57="57" attr58="58" attr59="59" attr60="60" attr61="61" attr62="62" attr63="63" attr64="64" attr65="65" attr66="66" attr67="67" attr68="68" attr69="69" attr70="70" attr71="71" attr72="72" attr73="73" attr74="74" attr75="75" attr76="76" attr77="77" attr78="78" attr79="79" attr80="80" attr81="81" attr82="82" attr83="83" attr84="84" attr85="85" attr86="86" attr87="87" attr88="88" attr89="89" attr90="90" attr91="91" attr92="92" attr93="93" attr94="94" attr95="95" attr96="96" attr97="97" attr98="98" attr99="99" attr100="100" attr101="101" attr102="102" attr103="103" attr104="104" attr105="105" attr106="106" attr107="107" attr108="108" attr109="109" attr110="110" attr111="111" attr112="112" attr113="113" attr114="114" attr115="115" attr116="116" attr117="117" attr118="118" attr119="119" attr120="120" attr121="121" attr122="122" attr123="123" attr124="124" attr125="125" attr126="126" attr127="127" attr128="128" attr129="129" attr130="130" attr131="131" attr132="132" attr133="133" attr134="134" attr135="135" attr136="136" attr137="137" attr138="138" attr139="139" attr140="140" attr141="141" attr142="142" attr143="143" attr144="144" attr145="145" attr146="146" attr147="147" attr148="148" attr149="149" attr150="150" attr151="151" attr152="152" attr153="153" attr154="154" attr155="155" attr156="156" attr157="157" attr158="158" attr159="159" attr160="160" attr161="161" attr162="162" attr163="163" attr164="164" attr165="165" attr166="166" attr167="167" attr168="168" attr169="169" attr170="170" attr171="171" attr172="172" attr173="173" attr174="174" attr175="175" attr176="176" attr177="177" attr178="178" attr179="179" attr180="180" attr181="181" attr182="182" attr183="183" attr184="184" attr185="185" attr186="186" attr187="187" attr188="188" attr189="189" attr190="190" attr191="191" attr192="192" attr193="193" attr194="194" attr195="195" attr196="196" attr197="197" attr198="198" attr199="199" attr200="200" attr201="201" attr202="202" attr203="203" attr204="204" attr205="205" attr206="206" attr207="207" attr208="208" attr209="209" attr210="210" attr211="211" attr212="212" attr213="213" attr214="214" attr215="215" attr216="216" attr217="217" attr218="218" attr219="219" attr220="220" attr221="221" attr222="222" attr223="223" attr224="224" attr225="225" attr226="226" attr227="227" attr228="228" attr229="229" attr230="230" attr231="231" attr232="232" attr233="233" attr234="234" attr235="235" attr236="236" attr237="237" attr238="238" attr239="239" attr240="240" attr241="241" attr242="242" attr243="243" attr244="244" attr245="245" attr246="246" attr247="247" attr248="248" attr249="249" attr250="250" attr251="251" attr252="252" attr253="253" attr254="254" attr255="255" attr256="256" attr257="257" attr258="258" attr259="259" attr260="260" attr261="261" attr262="262" attr263="263" attr264="264" attr265="265" attr266="266" attr267="267" attr268="268" attr269="269" attr270="270" attr271="271" attr272="272" attr273="273" attr274="274" attr275="275" attr276="276" attr277="277" attr278="278" attr279="279" attr280="280" attr281="281" attr282="282" attr283="283" attr284="284" attr285="285" attr286="286" attr287="287" attr288="288" attr289="289" attr290="290" attr291="291" attr292="292" attr293="293" attr294="294" attr295="295" attr296="296" attr297="297" attr298="298" attr299="299" attr300="300" attr301="301" attr302="302" attr303="303" attr304="304" attr305="305" attr306="306" attr307="307" attr308="308" attr309="309" attr310="310" attr311="311" attr312="312" attr313="313" attr314="314" attr315="315" attr316="316" attr317="317" attr318="318" attr319="319" attr320="320" attr321="321" attr322="322" attr323="323" attr324="324" attr325="325" attr326="326" attr327="327" attr328="328" attr329="329" attr330="330" attr331="331" attr332="332" attr333="333" attr334="334" attr335="335" attr336="336" attr337="337" attr338="338" attr339="339" attr340="340" attr341="341" attr342="342" attr343="343" attr344="344" attr345="345" attr346="346" attr347="347" attr348="348" attr349="349" attr350="350" attr351="351" attr352="352" attr353="353" attr354="354" attr355="355" attr356="356" attr357="357" attr358="358" attr359="359" attr360="360" attr361="361" attr362="362" attr363="363" attr364="364" attr365="365" attr366="366" attr367="367" attr368="368" attr369="369" attr370="370" attr371="371" attr372="372" attr373="373" attr374="374" attr375="375" attr376="376" attr377="377" attr378="378" attr379="379" attr380="380" attr381="381" attr382="382" attr383="383" attr384="384" attr385="385" attr386="386" attr387="387" attr388="388" attr389="389" attr390="390" attr391="391" attr392="392" attr393="393" attr394="394" attr395="395" attr396="396" attr397="397" attr398="398" attr399="399" attr400="400" attr401="401" attr402="402" attr403="403" attr404="404" attr405="405" attr406="406" attr407="407" attr408="408" attr409="409" attr410="410" attr411="411" attr412="412" attr413="413" attr414="414" attr415="415" attr416="416" attr417="417" attr418="418" attr419="419" attr420="420" attr421="421" attr422="422" attr423="423" attr424="424" attr425="425" attr426="426" attr427="427" attr428="428" attr429="429" attr430="430" attr431="431" attr432="432" attr433="433" attr434="434" attr435="435" attr436="436" attr437="437" attr438="438" attr439="439" attr440="440" attr441="441" attr442="442" attr443="443" attr444="444" attr445="445" attr446="446" attr447="447" attr448="448" attr449="449" attr450="450" attr451="451" attr452="452" attr453="453" attr454="454" attr455="455" attr456="456" attr457="457" attr458="458" attr459="459" attr460="460" attr461="461" attr462="462" attr463="463" attr464="464" attr465="465" attr466="466" attr467="467" attr468="468" attr469="469" attr470="470" attr471="471" attr472="472" attr473="473" attr474="474" attr475="475" attr476="476" attr477="477" attr478="478" attr479="479" attr480="480" attr481="481" attr482="482" attr483="483" attr484="484" attr485="485" attr486="486" attr487="487" attr488="488" attr489="489" attr490="490" attr491="491" attr492="492" attr493="493" attr494="494" attr495="495" attr496="496" attr497="497" attr498="498" attr499="499" attr500="500" attr501="501" attr502="502" attr503="503" attr504="504" attr505="505" attr506="506" attr507="507" attr508="508" attr509="509" attr510="510" attr511="511" attr512="512" attr513="513" attr514="514" attr515="515" attr516="516" attr517="517" attr518="518" attr519="519" attr520="520" attr521="521" attr522="522" attr523="523" attr524="524" attr525="525" attr526="526" attr527="527" attr528="528" attr529="529" attr530="530" attr531="531" attr532="532" attr533="533" attr534="534" attr535="535" attr536="536" attr537="537" attr538="538" attr539="539" attr540="540" attr541="541" attr542="542" attr543="543" attr544="544" attr545="545" attr546="546" attr547="547" attr548="548" attr549="549" attr550="550" attr551="551" attr552="552" attr553="553" attr554="554" attr555="555" attr556="556" attr557="557" attr558="558" attr559="559" attr560="560" attr561="561" attr562="562" attr563="563" attr564="564" attr565="565" attr566="566" attr567="567" attr568="568" attr569="569" attr570="570" attr571="571" attr572="572" attr573="573" attr574="574" attr575="575" attr576="576" attr577="577" attr578="578" attr579="579" attr580="580" attr581="581" attr582="582" attr583="583" attr584="584" attr585="585" attr586="586" attr587="587" attr588="588" attr589="589" attr590="590" attr591="591" attr592="592" attr593="593" attr594="594" attr595="595" attr596="596" attr597="597" attr598="598" attr599="599" attr600="600" attr601="601" attr602="602" attr603="603" attr604="604" attr605="605" attr606="606" attr607="607" attr608="608" attr609="609" attr610="610" attr611="611" attr612="612" attr613="613" attr614="614" attr615="615" attr616="616" attr617="617" attr618="618" attr619="619" attr620="620" attr621="621" attr622="622" attr623="623" attr624="624" attr625="625" attr626="626" attr627="627" attr628="628" attr629="629" attr630="630" attr631="631" attr632="632" attr633="633" attr634="634" attr635="635" attr636="636" attr637="637" attr638="638" attr639="639" attr640="640" attr641="641" attr642="642" attr643="643" attr644="644" attr645="645" attr646="646" attr647="647" attr648="648" attr649="649" attr650="650" attr651="651" attr652="652" attr653="653" attr654="654" attr655="655" attr656="656" attr657="657" attr658="658" attr659="659" attr660="660" attr661="661" attr662="662" attr663="663" attr664="664" attr665="665" attr666="666" attr667="667" attr668="668" attr669="669" attr670="670" attr671="671" attr672="672" attr673="673" attr674="674" attr675="675" attr676="676" attr677="677" attr678="678" attr679="679" attr680="680" attr681="681" attr682="682" attr683="683" attr684="684" attr685="685" attr686="686" attr687="687" attr688="688" attr689="689" attr690="690" attr691="691" attr692="692" attr693="693" attr694="694" attr695="695" attr696="696" attr697="697" attr698="698" attr699="699" attr700="700" attr701="701" attr702="702" attr703="703" attr704="704" attr705="705" attr706="706" attr707="707" attr708="708" attr709="709" attr710="710" attr711="711" attr712="712" attr713="713" attr714="714" attr715="715" attr716="716" attr717="717" attr718="718" attr719="719" attr720="720" attr721="721" attr722="722" attr723="723" attr724="724" attr725="725" attr726="726" attr727="727" attr728="728" attr729="729" attr730="730" attr731="731" attr732="732" attr733="733" attr734="734" attr735="735" attr736="736" attr737="737" attr738="738" attr739="739" attr740="740" attr741="741" attr742="742" attr743="743" attr744="744" attr745="745" attr746="746" attr747="747" attr748="748" attr749="749" attr750="750" attr751="751" attr752="752" attr753="753" attr754="754" attr755="755" attr756="756" attr757="757" attr758="758" attr759="759" attr760="760" attr761="761" attr762="762" attr763="763" attr764="764" attr765="765" attr766="766" attr767="767" attr768="768" attr769="769" attr770="770" attr771="771" attr772="772" attr773="773" attr774="774" attr775="775" attr776="776" attr777="777" attr778="778" attr779="779" attr780="780" attr781="781" attr782="782" attr783="783" attr784="784" attr785="785" attr786="786" attr787="787" attr788="788" attr789="789" attr790="790" attr791="791" attr792="792" attr793="793" attr794="794" attr795="795" attr796="796" attr797="797" attr798="798" attr799="799" attr800="800" attr801="801" attr802="802" attr803="803" attr804="804" attr805="805" attr806="806" attr807="807" attr808="808" attr809="809" attr810="810" attr811="811" attr812="812" attr813="813" attr814="814" attr815="815" attr816="816" attr817="817" attr818="818" attr819="819" attr820="820" attr821="821" attr822="822" attr823="823" attr824="824" attr825="825" attr826="826" attr827="827" attr828="828" attr829="829" attr830="830" attr831="831" attr832="832" attr833="833" attr834="834" attr835="835" attr836="836" attr837="837" attr838="838" attr839="839" attr840="840" attr841="841" attr842="842" attr843="843" attr844="844" attr845="845" attr846="846" attr847="847" attr848="848" attr849="849" attr850="850" attr851="851" attr852="852" attr853="853" attr854="854" attr855="855" attr856="856" attr857="857" attr858="858" attr859="859" attr860="860" attr861="861" attr862="862" attr863="863" attr864="864" attr865="865" attr866="866" attr867="867" attr868="868" attr869="869" attr870="870" attr871="871" attr872="872" attr873="873" attr874="874" attr875="875" attr876="876" attr877="877" attr878="878" attr879="879" attr880="880" attr881="881" attr882="882" attr883="883" attr884="884" attr885="885" attr886="886" attr887="887" attr888="888" attr889="889" attr890="890" attr891="891" attr892="892" attr893="893" attr894="894" attr895="895" attr896="896" attr897="897" attr898="898" attr899="899" attr900="900" attr901="901" attr902="902" attr903="903" attr904="904" attr905="905" attr906="906" attr907="907" attr908="908" attr909="909" attr910="910" attr911="911" attr912="912" attr913="913" attr914="914" attr915="915" attr916="916" attr917="917" attr918="918" attr919="919" attr920="920" attr921="921" attr922="922" attr923="923" attr924="924" attr925="925" attr926="926" attr927="927" attr928="928" attr929="929" attr930="930" attr931="931" attr932="932" attr933="933" attr934="934" attr935="935" attr936="936" attr937="937" attr938="938" attr939="939" attr940="940" attr941="941" attr942="942" attr943="943" attr944="944" attr945="945" attr946="946" attr947="947" attr948="948" attr949="949" attr950="950" attr951="951" attr952="952" attr953="953" attr954="954" attr955="955" attr956="956" attr957="957" attr958="958" attr959="959" attr960="960" attr961="961" attr962="962" attr963="963" attr964="964" attr965="965" attr966="966" attr967="967" attr968="968" attr969="969" attr970="970" attr971="971" attr972="972" attr973="973" attr974="974" attr975="975" attr976="976" attr977="977" attr978="978" attr979="979" attr980="980" attr981="981" attr982="982" attr983="983" attr984="984" attr985="985" attr986="986" attr987="987" attr988="988" attr989="989" attr990="990" attr991="991" attr992="992" attr993="993" attr994="994" attr995="995" attr996="996" attr997="997" attr998="998" attr999="999" attr1000="1000">
                "nice"
            </h1>
        }
    })
}

fn html_node_many_attrs() -> String {
    html_node::html! {
        <h1 attr1="1" attr2="2" attr3="3" attr4="4" attr5="5" attr6="6" attr7="7" attr8="8" attr9="9" attr10="10" attr11="11" attr12="12" attr13="13" attr14="14" attr15="15" attr16="16" attr17="17" attr18="18" attr19="19" attr20="20" attr21="21" attr22="22" attr23="23" attr24="24" attr25="25" attr26="26" attr27="27" attr28="28" attr29="29" attr30="30" attr31="31" attr32="32" attr33="33" attr34="34" attr35="35" attr36="36" attr37="37" attr38="38" attr39="39" attr40="40" attr41="41" attr42="42" attr43="43" attr44="44" attr45="45" attr46="46" attr47="47" attr48="48" attr49="49" attr50="50" attr51="51" attr52="52" attr53="53" attr54="54" attr55="55" attr56="56" attr57="57" attr58="58" attr59="59" attr60="60" attr61="61" attr62="62" attr63="63" attr64="64" attr65="65" attr66="66" attr67="67" attr68="68" attr69="69" attr70="70" attr71="71" attr72="72" attr73="73" attr74="74" attr75="75" attr76="76" attr77="77" attr78="78" attr79="79" attr80="80" attr81="81" attr82="82" attr83="83" attr84="84" attr85="85" attr86="86" attr87="87" attr88="88" attr89="89" attr90="90" attr91="91" attr92="92" attr93="93" attr94="94" attr95="95" attr96="96" attr97="97" attr98="98" attr99="99" attr100="100" attr101="101" attr102="102" attr103="103" attr104="104" attr105="105" attr106="106" attr107="107" attr108="108" attr109="109" attr110="110" attr111="111" attr112="112" attr113="113" attr114="114" attr115="115" attr116="116" attr117="117" attr118="118" attr119="119" attr120="120" attr121="121" attr122="122" attr123="123" attr124="124" attr125="125" attr126="126" attr127="127" attr128="128" attr129="129" attr130="130" attr131="131" attr132="132" attr133="133" attr134="134" attr135="135" attr136="136" attr137="137" attr138="138" attr139="139" attr140="140" attr141="141" attr142="142" attr143="143" attr144="144" attr145="145" attr146="146" attr147="147" attr148="148" attr149="149" attr150="150" attr151="151" attr152="152" attr153="153" attr154="154" attr155="155" attr156="156" attr157="157" attr158="158" attr159="159" attr160="160" attr161="161" attr162="162" attr163="163" attr164="164" attr165="165" attr166="166" attr167="167" attr168="168" attr169="169" attr170="170" attr171="171" attr172="172" attr173="173" attr174="174" attr175="175" attr176="176" attr177="177" attr178="178" attr179="179" attr180="180" attr181="181" attr182="182" attr183="183" attr184="184" attr185="185" attr186="186" attr187="187" attr188="188" attr189="189" attr190="190" attr191="191" attr192="192" attr193="193" attr194="194" attr195="195" attr196="196" attr197="197" attr198="198" attr199="199" attr200="200" attr201="201" attr202="202" attr203="203" attr204="204" attr205="205" attr206="206" attr207="207" attr208="208" attr209="209" attr210="210" attr211="211" attr212="212" attr213="213" attr214="214" attr215="215" attr216="216" attr217="217" attr218="218" attr219="219" attr220="220" attr221="221" attr222="222" attr223="223" attr224="224" attr225="225" attr226="226" attr227="227" attr228="228" attr229="229" attr230="230" attr231="231" attr232="232" attr233="233" attr234="234" attr235="235" attr236="236" attr237="237" attr238="238" attr239="239" attr240="240" attr241="241" attr242="242" attr243="243" attr244="244" attr245="245" attr246="246" attr247="247" attr248="248" attr249="249" attr250="250" attr251="251" attr252="252" attr253="253" attr254="254" attr255="255" attr256="256" attr257="257" attr258="258" attr259="259" attr260="260" attr261="261" attr262="262" attr263="263" attr264="264" attr265="265" attr266="266" attr267="267" attr268="268" attr269="269" attr270="270" attr271="271" attr272="272" attr273="273" attr274="274" attr275="275" attr276="276" attr277="277" attr278="278" attr279="279" attr280="280" attr281="281" attr282="282" attr283="283" attr284="284" attr285="285" attr286="286" attr287="287" attr288="288" attr289="289" attr290="290" attr291="291" attr292="292" attr293="293" attr294="294" attr295="295" attr296="296" attr297="297" attr298="298" attr299="299" attr300="300" attr301="301" attr302="302" attr303="303" attr304="304" attr305="305" attr306="306" attr307="307" attr308="308" attr309="309" attr310="310" attr311="311" attr312="312" attr313="313" attr314="314" attr315="315" attr316="316" attr317="317" attr318="318" attr319="319" attr320="320" attr321="321" attr322="322" attr323="323" attr324="324" attr325="325" attr326="326" attr327="327" attr328="328" attr329="329" attr330="330" attr331="331" attr332="332" attr333="333" attr334="334" attr335="335" attr336="336" attr337="337" attr338="338" attr339="339" attr340="340" attr341="341" attr342="342" attr343="343" attr344="344" attr345="345" attr346="346" attr347="347" attr348="348" attr349="349" attr350="350" attr351="351" attr352="352" attr353="353" attr354="354" attr355="355" attr356="356" attr357="357" attr358="358" attr359="359" attr360="360" attr361="361" attr362="362" attr363="363" attr364="364" attr365="365" attr366="366" attr367="367" attr368="368" attr369="369" attr370="370" attr371="371" attr372="372" attr373="373" attr374="374" attr375="375" attr376="376" attr377="377" attr378="378" attr379="379" attr380="380" attr381="381" attr382="382" attr383="383" attr384="384" attr385="385" attr386="386" attr387="387" attr388="388" attr389="389" attr390="390" attr391="391" attr392="392" attr393="393" attr394="394" attr395="395" attr396="396" attr397="397" attr398="398" attr399="399" attr400="400" attr401="401" attr402="402" attr403="403" attr404="404" attr405="405" attr406="406" attr407="407" attr408="408" attr409="409" attr410="410" attr411="411" attr412="412" attr413="413" attr414="414" attr415="415" attr416="416" attr417="417" attr418="418" attr419="419" attr420="420" attr421="421" attr422="422" attr423="423" attr424="424" attr425="425" attr426="426" attr427="427" attr428="428" attr429="429" attr430="430" attr431="431" attr432="432" attr433="433" attr434="434" attr435="435" attr436="436" attr437="437" attr438="438" attr439="439" attr440="440" attr441="441" attr442="442" attr443="443" attr444="444" attr445="445" attr446="446" attr447="447" attr448="448" attr449="449" attr450="450" attr451="451" attr452="452" attr453="453" attr454="454" attr455="455" attr456="456" attr457="457" attr458="458" attr459="459" attr460="460" attr461="461" attr462="462" attr463="463" attr464="464" attr465="465" attr466="466" attr467="467" attr468="468" attr469="469" attr470="470" attr471="471" attr472="472" attr473="473" attr474="474" attr475="475" attr476="476" attr477="477" attr478="478" attr479="479" attr480="480" attr481="481" attr482="482" attr483="483" attr484="484" attr485="485" attr486="486" attr487="487" attr488="488" attr489="489" attr490="490" attr491="491" attr492="492" attr493="493" attr494="494" attr495="495" attr496="496" attr497="497" attr498="498" attr499="499" attr500="500" attr501="501" attr502="502" attr503="503" attr504="504" attr505="505" attr506="506" attr507="507" attr508="508" attr509="509" attr510="510" attr511="511" attr512="512" attr513="513" attr514="514" attr515="515" attr516="516" attr517="517" attr518="518" attr519="519" attr520="520" attr521="521" attr522="522" attr523="523" attr524="524" attr525="525" attr526="526" attr527="527" attr528="528" attr529="529" attr530="530" attr531="531" attr532="532" attr533="533" attr534="534" attr535="535" attr536="536" attr537="537" attr538="538" attr539="539" attr540="540" attr541="541" attr542="542" attr543="543" attr544="544" attr545="545" attr546="546" attr547="547" attr548="548" attr549="549" attr550="550" attr551="551" attr552="552" attr553="553" attr554="554" attr555="555" attr556="556" attr557="557" attr558="558" attr559="559" attr560="560" attr561="561" attr562="562" attr563="563" attr564="564" attr565="565" attr566="566" attr567="567" attr568="568" attr569="569" attr570="570" attr571="571" attr572="572" attr573="573" attr574="574" attr575="575" attr576="576" attr577="577" attr578="578" attr579="579" attr580="580" attr581="581" attr582="582" attr583="583" attr584="584" attr585="585" attr586="586" attr587="587" attr588="588" attr589="589" attr590="590" attr591="591" attr592="592" attr593="593" attr594="594" attr595="595" attr596="596" attr597="597" attr598="598" attr599="599" attr600="600" attr601="601" attr602="602" attr603="603" attr604="604" attr605="605" attr606="606" attr607="607" attr608="608" attr609="609" attr610="610" attr611="611" attr612="612" attr613="613" attr614="614" attr615="615" attr616="616" attr617="617" attr618="618" attr619="619" attr620="620" attr621="621" attr622="622" attr623="623" attr624="624" attr625="625" attr626="626" attr627="627" attr628="628" attr629="629" attr630="630" attr631="631" attr632="632" attr633="633" attr634="634" attr635="635" attr636="636" attr637="637" attr638="638" attr639="639" attr640="640" attr641="641" attr642="642" attr643="643" attr644="644" attr645="645" attr646="646" attr647="647" attr648="648" attr649="649" attr650="650" attr651="651" attr652="652" attr653="653" attr654="654" attr655="655" attr656="656" attr657="657" attr658="658" attr659="659" attr660="660" attr661="661" attr662="662" attr663="663" attr664="664" attr665="665" attr666="666" attr667="667" attr668="668" attr669="669" attr670="670" attr671="671" attr672="672" attr673="673" attr674="674" attr675="675" attr676="676" attr677="677" attr678="678" attr679="679" attr680="680" attr681="681" attr682="682" attr683="683" attr684="684" attr685="685" attr686="686" attr687="687" attr688="688" attr689="689" attr690="690" attr691="691" attr692="692" attr693="693" attr694="694" attr695="695" attr696="696" attr697="697" attr698="698" attr699="699" attr700="700" attr701="701" attr702="702" attr703="703" attr704="704" attr705="705" attr706="706" attr707="707" attr708="708" attr709="709" attr710="710" attr711="711" attr712="712" attr713="713" attr714="714" attr715="715" attr716="716" attr717="717" attr718="718" attr719="719" attr720="720" attr721="721" attr722="722" attr723="723" attr724="724" attr725="725" attr726="726" attr727="727" attr728="728" attr729="729" attr730="730" attr731="731" attr732="732" attr733="733" attr734="734" attr735="735" attr736="736" attr737="737" attr738="738" attr739="739" attr740="740" attr741="741" attr742="742" attr743="743" attr744="744" attr745="745" attr746="746" attr747="747" attr748="748" attr749="749" attr750="750" attr751="751" attr752="752" attr753="753" attr754="754" attr755="755" attr756="756" attr757="757" attr758="758" attr759="759" attr760="760" attr761="761" attr762="762" attr763="763" attr764="764" attr765="765" attr766="766" attr767="767" attr768="768" attr769="769" attr770="770" attr771="771" attr772="772" attr773="773" attr774="774" attr775="775" attr776="776" attr777="777" attr778="778" attr779="779" attr780="780" attr781="781" attr782="782" attr783="783" attr784="784" attr785="785" attr786="786" attr787="787" attr788="788" attr789="789" attr790="790" attr791="791" attr792="792" attr793="793" attr794="794" attr795="795" attr796="796" attr797="797" attr798="798" attr799="799" attr800="800" attr801="801" attr802="802" attr803="803" attr804="804" attr805="805" attr806="806" attr807="807" attr808="808" attr809="809" attr810="810" attr811="811" attr812="812" attr813="813" attr814="814" attr815="815" attr816="816" attr817="817" attr818="818" attr819="819" attr820="820" attr821="821" attr822="822" attr823="823" attr824="824" attr825="825" attr826="826" attr827="827" attr828="828" attr829="829" attr830="830" attr831="831" attr832="832" attr833="833" attr834="834" attr835="835" attr836="836" attr837="837" attr838="838" attr839="839" attr840="840" attr841="841" attr842="842" attr843="843" attr844="844" attr845="845" attr846="846" attr847="847" attr848="848" attr849="849" attr850="850" attr851="851" attr852="852" attr853="853" attr854="854" attr855="855" attr856="856" attr857="857" attr858="858" attr859="859" attr860="860" attr861="861" attr862="862" attr863="863" attr864="864" attr865="865" attr866="866" attr867="867" attr868="868" attr869="869" attr870="870" attr871="871" attr872="872" attr873="873" attr874="874" attr875="875" attr876="876" attr877="877" attr878="878" attr879="879" attr880="880" attr881="881" attr882="882" attr883="883" attr884="884" attr885="885" attr886="886" attr887="887" attr888="888" attr889="889" attr890="890" attr891="891" attr892="892" attr893="893" attr894="894" attr895="895" attr896="896" attr897="897" attr898="898" attr899="899" attr900="900" attr901="901" attr902="902" attr903="903" attr904="904" attr905="905" attr906="906" attr907="907" attr908="908" attr909="909" attr910="910" attr911="911" attr912="912" attr913="913" attr914="914" attr915="915" attr916="916" attr917="917" attr918="918" attr919="919" attr920="920" attr921="921" attr922="922" attr923="923" attr924="924" attr925="925" attr926="926" attr927="927" attr928="928" attr929="929" attr930="930" attr931="931" attr932="932" attr933="933" attr934="934" attr935="935" attr936="936" attr937="937" attr938="938" attr939="939" attr940="940" attr941="941" attr942="942" attr943="943" attr944="944" attr945="945" attr946="946" attr947="947" attr948="948" attr949="949" attr950="950" attr951="951" attr952="952" attr953="953" attr954="954" attr955="955" attr956="956" attr957="957" attr958="958" attr959="959" attr960="960" attr961="961" attr962="962" attr963="963" attr964="964" attr965="965" attr966="966" attr967="967" attr968="968" attr969="969" attr970="970" attr971="971" attr972="972" attr973="973" attr974="974" attr975="975" attr976="976" attr977="977" attr978="978" attr979="979" attr980="980" attr981="981" attr982="982" attr983="983" attr984="984" attr985="985" attr986="986" attr987="987" attr988="988" attr989="989" attr990="990" attr991="991" attr992="992" attr993="993" attr994="994" attr995="995" attr996="996" attr997="997" attr998="998" attr999="999" attr1000="1000">
            "nice"
        </h1>
    }.to_string()
}

/// small fragment with some attributes and props

fn maud_small_fragment() -> String {
    maud::html! {
        div class="some-class" customattr="42" {
            (maud_hero_title("great-stuff".to_string(), 10, "boh".to_string()))
        }
    }
    .0
}

fn maud_hero_title(class: String, _xxx: i32, _yyy: String) -> maud::Markup {
    maud::html! {
        h1 {
            "my class is " (class)
        }
    }
}

async fn rscx_small_fragment() -> String {
    rscx::html! {
        <div class="some-class" customattr=42>
            <RscHeroTitle class="great-stuff".to_string() xxx=10 yyy="boh".to_string()>
            </RscHeroTitle>
        </div>
    }
}

#[rscx::props]
pub struct RscHeroTitleProps {
    class: String,
    xxx: i32,
    yyy: String,
}

#[rscx::component]
fn RscHeroTitle(props: RscHeroTitleProps) -> String {
    rscx::html! {
        <h1>my class is {props.class}</h1>
    }
}

fn leptos_small_fragment() -> leptos::Oco<'static, str> {
    leptos::ssr::render_to_string(|| {
        leptos::view! {
            <div class="some-class" customattr=42>
                <LeptosHeroTitle class="great-stuff".to_string() xxx=10 yyy="boh".to_string()>
                </LeptosHeroTitle>
            </div>
        }
    })
}

#[leptos::component]
#[allow(unused_variables)]
pub fn LeptosHeroTitle(class: String, xxx: i32, yyy: String) -> impl IntoView {
    leptos::view! {
        <h1>my class is {class}</h1>
    }
}

// async data
async fn rscx_async_list_app() -> String {
    rscx::html! {
        <div>
            <RscAsyncList />
        </div>
    }
}

#[rscx::component]
async fn RscAsyncList() -> String {
    let data = load_data_async().await;
    use rscx::CollectFragment;
    rscx::html! {
        <ul>
            {
                data
                    .into_iter()
                    .map(|item| rscx::html! { <li>{ item }</li> })
                    .collect_fragment()
            }
        </ul>
    }
}

async fn leptos_async_list_app() -> String {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async move {
            let (stream, runtime) =
                leptos::ssr::render_to_stream_in_order_with_prefix_undisposed_with_context(
                    || {
                        leptos::view! {
                            <div>
                                <LeptosAsyncList />
                            </div>
                        }
                        .into_view()
                    },
                    || "".into(),
                    || {},
                );
            let mut stream = Box::pin(stream);
            let mut buf = String::new();
            while let Some(chunk) = stream.next().await {
                buf.push_str(&chunk);
            }
            runtime.dispose();
            buf
        })
        .await
}

#[leptos::component]
fn LeptosAsyncList() -> impl IntoView {
    let once = leptos::create_resource(|| (), move |()| load_data_async());

    use leptos::{CollectView, SignalGet, Suspense};
    leptos::view! {
        <ul>
            <Suspense fallback={|| ()}>
                { move || once.get()
                    .map(|items|
                        items
                            .into_iter()
                            .map(|item| leptos::view! { <li>{ item }</li> })
                            .collect_view()
                    ) }
            </Suspense>
        </ul>
    }
}

async fn maud_async_list_app() -> String {
    maud::html! {
        div {
            (maud_async_list().await)
        }
    }
    .0
}

async fn maud_async_list() -> maud::Markup {
    let data = load_data_async().await;
    maud::html! {
        ul {
            @for item in data {
                li { (item) }
            }
        }
    }
}

async fn load_data_async() -> Vec<String> {
    vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
        "10".to_string(),
        "11".to_string(),
        "12".to_string(),
        "13".to_string(),
        "14".to_string(),
        "15".to_string(),
        "16".to_string(),
        "17".to_string(),
        "18".to_string(),
        "19".to_string(),
        "20".to_string(),
        "21".to_string(),
        "22".to_string(),
        "23".to_string(),
        "24".to_string(),
        "25".to_string(),
        "26".to_string(),
        "27".to_string(),
        "28".to_string(),
        "29".to_string(),
        "30".to_string(),
        "31".to_string(),
        "32".to_string(),
        "33".to_string(),
        "34".to_string(),
        "35".to_string(),
        "36".to_string(),
        "37".to_string(),
        "38".to_string(),
        "39".to_string(),
        "40".to_string(),
        "41".to_string(),
        "42".to_string(),
        "43".to_string(),
        "44".to_string(),
        "45".to_string(),
        "46".to_string(),
        "47".to_string(),
        "48".to_string(),
        "49".to_string(),
        "50".to_string(),
        "51".to_string(),
        "52".to_string(),
        "53".to_string(),
        "54".to_string(),
        "55".to_string(),
        "56".to_string(),
        "57".to_string(),
        "58".to_string(),
        "59".to_string(),
        "60".to_string(),
        "61".to_string(),
        "62".to_string(),
        "63".to_string(),
        "64".to_string(),
        "65".to_string(),
        "66".to_string(),
        "67".to_string(),
        "68".to_string(),
        "69".to_string(),
        "70".to_string(),
        "71".to_string(),
        "72".to_string(),
        "73".to_string(),
        "74".to_string(),
        "75".to_string(),
        "76".to_string(),
        "77".to_string(),
        "78".to_string(),
        "79".to_string(),
        "80".to_string(),
        "81".to_string(),
        "82".to_string(),
        "83".to_string(),
        "84".to_string(),
        "85".to_string(),
        "86".to_string(),
        "87".to_string(),
        "88".to_string(),
        "89".to_string(),
        "90".to_string(),
        "91".to_string(),
        "92".to_string(),
        "93".to_string(),
        "94".to_string(),
        "95".to_string(),
        "96".to_string(),
        "97".to_string(),
        "98".to_string(),
        "99".to_string(),
        "100".to_string(),
    ]
}
