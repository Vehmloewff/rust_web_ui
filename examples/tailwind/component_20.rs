use rust_web_ui::*;

pub struct Example20;

pub struct Example20Props {}

impl Default for Example20Props {
	fn default() -> Example20Props {
		Example20Props { }
	}
}

impl Widget<'_> for Example20 {
	type Props = Example20Props;

	fn render(mut ctx: Ctx<'_>, props: Example20Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("min-h-[583px]"), NoStyle::Noop("overflow-auto"), NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.set_attribute("x-data", "{ open: false }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.styles(&[NoStyle::Noop("bg-white")]);
				
				//  Mobile menu 
				props.child("3", Dynamic).run("div", |props| {
					props.set_attribute("x-show", "open");
					props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-40"), Screen::Large(&[NoStyle::Noop("hidden")])]);
					props.set_attribute("x-description", "Off-canvas menu for mobile, show/hide based on off-canvas menu state.");
					props.set_attribute("x-ref", "dialog");
					props.set_attribute("aria-modal", "true");
					
					props.child("1", Dynamic).run("div", |props| {
						props.set_attribute("x-show", "open");
						props.set_attribute("x-transition:enter", "transition-opacity ease-linear duration-300");
						props.set_attribute("x-transition:enter-start", "opacity-0");
						props.set_attribute("x-transition:enter-end", "opacity-100");
						props.set_attribute("x-transition:leave", "transition-opacity ease-linear duration-300");
						props.set_attribute("x-transition:leave-start", "opacity-100");
						props.set_attribute("x-transition:leave-end", "opacity-0");
						props.set_attribute("x-description", "Off-canvas menu backdrop, show/hide based on off-canvas menu state.");
						props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("bg-black"), NoStyle::Noop("bg-opacity-25")]);
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-40"), Style::Flex]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.set_attribute("x-show", "open");
							props.set_attribute("x-transition:enter", "transition ease-in-out duration-300 transform");
							props.set_attribute("x-transition:enter-start", "-translate-x-full");
							props.set_attribute("x-transition:enter-end", "translate-x-0");
							props.set_attribute("x-transition:leave", "transition ease-in-out duration-300 transform");
							props.set_attribute("x-transition:leave-start", "translate-x-0");
							props.set_attribute("x-transition:leave-end", "-translate-x-full");
							props.set_attribute("x-description", "Off-canvas menu, show/hide based on off-canvas menu state.");
							props.styles(&[NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("w-full"), NoStyle::Noop("max-w-xs"), NoStyle::Noop("flex-col"), NoStyle::Noop("overflow-y-auto"), NoStyle::Noop("bg-white"), Style::PaddingBottom(48), NoStyle::Noop("shadow-xl")]);
							props.set_attribute("@click.away", "open = false");
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::Flex, Style::PaddingX(16), Style::PaddingBottom(8), Style::PaddingTop(20)]);
								
								props.child("1", Dynamic).run("button", |props| {
									props.set_attribute("type", "button");
									props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("-m-2"), Style::InlineFlex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), Style::Padding(8), Style::TextColor(Color::Fg(44))]);
									props.set_attribute("@click", "open = false");
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("-inset-0.5")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.set_text("Close menu"));
									});
									props.child("5", Icon).run(|props| {
										props.style(&[Style::Width(24), Style::Width(24)]);
									});
								});
							});
							//  Links 
							props.child("5", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(8)]);
								props.set_attribute("x-data", "Components.tabs()");
								props.set_attribute("@tab-click.window", "onTabClick");
								props.set_attribute("@tab-keydown.window", "onTabKeydown");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-200")]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("-mb-px"), Style::Flex, Style::SpaceX(32), Style::PaddingX(16)]);
										props.set_attribute("aria-orientation", "horizontal");
										props.set_attribute("role", "tablist");
										
										props.child("1", Dynamic).run("button", |props| {
											props.set_attribute("id", "tabs-1-tab-1");
											props.styles(&[NoStyle::Noop("border-transparent"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("flex-1"), NoStyle::Noop("whitespace-nowrap"), NoStyle::Noop("border-b-2"), Style::PaddingX(4), Style::PaddingY(16), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium")]);
											props.set_attribute("x-state:on", "Selected");
											props.set_attribute("x-state:off", "Not Selected");
											props.set_attribute(":class", "{ 'border-indigo-600 text-indigo-600': selected, 'border-transparent text-gray-900': !(selected) }");
											props.set_attribute("x-data", "Components.tab(0)");
											props.set_attribute("aria-controls", "tabs-1-panel-1");
											props.set_attribute("role", "tab");
											props.set_attribute("x-init", "init()");
											props.set_attribute("@click", "onClick");
											props.set_attribute("@keydown", "onKeydown");
											props.set_attribute("@tab-select.window", "onTabSelect");
											props.set_attribute(":tabindex", "selected ? 0 : -1");
											props.set_attribute(":aria-selected", "selected ? 'true' : 'false'");
											props.set_attribute("type", "button");
											
											props.child("0", Label).run(|props| props.set_text("Women"));
										});
										props.child("3", Dynamic).run("button", |props| {
											props.set_attribute("id", "tabs-1-tab-2");
											props.styles(&[NoStyle::Noop("border-transparent"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("flex-1"), NoStyle::Noop("whitespace-nowrap"), NoStyle::Noop("border-b-2"), Style::PaddingX(4), Style::PaddingY(16), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium")]);
											props.set_attribute("x-state:on", "Selected");
											props.set_attribute("x-state:off", "Not Selected");
											props.set_attribute(":class", "{ 'border-indigo-600 text-indigo-600': selected, 'border-transparent text-gray-900': !(selected) }");
											props.set_attribute("x-data", "Components.tab(0)");
											props.set_attribute("aria-controls", "tabs-1-panel-2");
											props.set_attribute("role", "tab");
											props.set_attribute("x-init", "init()");
											props.set_attribute("@click", "onClick");
											props.set_attribute("@keydown", "onKeydown");
											props.set_attribute("@tab-select.window", "onTabSelect");
											props.set_attribute(":tabindex", "selected ? 0 : -1");
											props.set_attribute(":aria-selected", "selected ? 'true' : 'false'");
											props.set_attribute("type", "button");
											
											props.child("0", Label).run(|props| props.set_text("Men"));
										});
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.set_attribute("id", "tabs-1-panel-1");
									props.styles(&[Style::SpaceY(40), Style::PaddingX(16), Style::PaddingBottom(32), Style::PaddingTop(40)]);
									props.set_attribute("x-description", "'Women' tab panel, show/hide based on tab state.");
									props.set_attribute("x-data", "Components.tabPanel(0)");
									props.set_attribute("aria-labelledby", "tabs-1-tab-1");
									props.set_attribute("x-init", "init()");
									props.set_attribute("x-show", "selected");
									props.set_attribute("@tab-select.window", "onTabSelect");
									props.set_attribute("role", "tabpanel");
									props.set_attribute("tabindex", "0");
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-2"), NoStyle::Noop("gap-x-4")]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), NoStyle::Noop("text-sm")]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/mega-menu-category-01.jpg");
													props.set_attribute("alt", "Models sitting back to back, wearing Basic Tee in black and bone.");
													props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::MarginTop(24), Style::Block, NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("2", Label).run(|props| props.set_text("New Arrivals"));
											});
											props.child("5", Dynamic).run("p", |props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::MarginTop(4)]);
												
												props.child("0", Label).run(|props| props.set_text("Shop now"));
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), NoStyle::Noop("text-sm")]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/mega-menu-category-02.jpg");
													props.set_attribute("alt", "Close up of Basic Tee fall bundle with off-white, ochre, olive, and black tees.");
													props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::MarginTop(24), Style::Block, NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("2", Label).run(|props| props.set_text("Basic Tees"));
											});
											props.child("5", Dynamic).run("p", |props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::MarginTop(4)]);
												
												props.child("0", Label).run(|props| props.set_text("Shop now"));
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("p", |props| {
											props.set_attribute("id", "women-clothing-heading-mobile");
											props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Clothing"));
										});
										props.child("3", Dynamic).run("ul", |props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "women-clothing-heading-mobile");
											props.styles(&[Style::MarginTop(24), Style::Flex, NoStyle::Noop("flex-col"), Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Tops"));
												});
											});
											props.child("3", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Dresses"));
												});
											});
											props.child("5", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Pants"));
												});
											});
											props.child("7", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Denim"));
												});
											});
											props.child("9", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Sweaters"));
												});
											});
											props.child("11", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("T-Shirts"));
												});
											});
											props.child("13", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Jackets"));
												});
											});
											props.child("15", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Activewear"));
												});
											});
											props.child("17", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Browse All"));
												});
											});
										});
									});
									props.child("5", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("p", |props| {
											props.set_attribute("id", "women-accessories-heading-mobile");
											props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Accessories"));
										});
										props.child("3", Dynamic).run("ul", |props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "women-accessories-heading-mobile");
											props.styles(&[Style::MarginTop(24), Style::Flex, NoStyle::Noop("flex-col"), Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Watches"));
												});
											});
											props.child("3", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Wallets"));
												});
											});
											props.child("5", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Bags"));
												});
											});
											props.child("7", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Sunglasses"));
												});
											});
											props.child("9", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Hats"));
												});
											});
											props.child("11", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Belts"));
												});
											});
										});
									});
									props.child("7", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("p", |props| {
											props.set_attribute("id", "women-brands-heading-mobile");
											props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Brands"));
										});
										props.child("3", Dynamic).run("ul", |props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "women-brands-heading-mobile");
											props.styles(&[Style::MarginTop(24), Style::Flex, NoStyle::Noop("flex-col"), Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Full Nelson"));
												});
											});
											props.child("3", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("My Way"));
												});
											});
											props.child("5", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Re-Arranged"));
												});
											});
											props.child("7", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Counterfeit"));
												});
											});
											props.child("9", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Significant Other"));
												});
											});
										});
									});
								});
								props.child("5", Dynamic).run("div", |props| {
									props.set_attribute("id", "tabs-1-panel-2");
									props.styles(&[Style::SpaceY(40), Style::PaddingX(16), Style::PaddingBottom(32), Style::PaddingTop(40)]);
									props.set_attribute("x-description", "'Men' tab panel, show/hide based on tab state.");
									props.set_attribute("x-data", "Components.tabPanel(0)");
									props.set_attribute("aria-labelledby", "tabs-1-tab-2");
									props.set_attribute("x-init", "init()");
									props.set_attribute("x-show", "selected");
									props.set_attribute("@tab-select.window", "onTabSelect");
									props.set_attribute("role", "tabpanel");
									props.set_attribute("tabindex", "0");
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-2"), NoStyle::Noop("gap-x-4")]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), NoStyle::Noop("text-sm")]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-04-detail-product-shot-01.jpg");
													props.set_attribute("alt", "Drawstring top with elastic loop closure and textured interior padding.");
													props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::MarginTop(24), Style::Block, NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("2", Label).run(|props| props.set_text("New Arrivals"));
											});
											props.child("5", Dynamic).run("p", |props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::MarginTop(4)]);
												
												props.child("0", Label).run(|props| props.set_text("Shop now"));
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), NoStyle::Noop("text-sm")]);
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
												
												props.child("1", Dynamic).run("img", |props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-02-image-card-06.jpg");
													props.set_attribute("alt", "Three shirts in gray, white, and blue arranged on table with same line drawing of hands and shapes overlapping on front of shirt.");
													props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::MarginTop(24), Style::Block, NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("2", Label).run(|props| props.set_text("Artwork Tees"));
											});
											props.child("5", Dynamic).run("p", |props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::MarginTop(4)]);
												
												props.child("0", Label).run(|props| props.set_text("Shop now"));
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("p", |props| {
											props.set_attribute("id", "men-clothing-heading-mobile");
											props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Clothing"));
										});
										props.child("3", Dynamic).run("ul", |props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "men-clothing-heading-mobile");
											props.styles(&[Style::MarginTop(24), Style::Flex, NoStyle::Noop("flex-col"), Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Tops"));
												});
											});
											props.child("3", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Pants"));
												});
											});
											props.child("5", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Sweaters"));
												});
											});
											props.child("7", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("T-Shirts"));
												});
											});
											props.child("9", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Jackets"));
												});
											});
											props.child("11", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Activewear"));
												});
											});
											props.child("13", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Browse All"));
												});
											});
										});
									});
									props.child("5", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("p", |props| {
											props.set_attribute("id", "men-accessories-heading-mobile");
											props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Accessories"));
										});
										props.child("3", Dynamic).run("ul", |props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "men-accessories-heading-mobile");
											props.styles(&[Style::MarginTop(24), Style::Flex, NoStyle::Noop("flex-col"), Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Watches"));
												});
											});
											props.child("3", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Wallets"));
												});
											});
											props.child("5", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Bags"));
												});
											});
											props.child("7", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Sunglasses"));
												});
											});
											props.child("9", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Hats"));
												});
											});
											props.child("11", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Belts"));
												});
											});
										});
									});
									props.child("7", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("p", |props| {
											props.set_attribute("id", "men-brands-heading-mobile");
											props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Brands"));
										});
										props.child("3", Dynamic).run("ul", |props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "men-brands-heading-mobile");
											props.styles(&[Style::MarginTop(24), Style::Flex, NoStyle::Noop("flex-col"), Style::SpaceY(24)]);
											
											props.child("1", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Re-Arranged"));
												});
											});
											props.child("3", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Counterfeit"));
												});
											});
											props.child("5", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("Full Nelson"));
												});
											});
											props.child("7", Dynamic).run("li", |props| {
												props.styles(&[NoStyle::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |props| {
													props.set_attribute("href", "#");
													props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.set_text("My Way"));
												});
											});
										});
									});
								});
							});
							props.child("7", Dynamic).run("div", |props| {
								props.styles(&[Style::SpaceY(24), NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingX(16), Style::PaddingY(24)]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("flow-root")]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Company"));
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("flow-root")]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Stores"));
									});
								});
							});
							props.child("9", Dynamic).run("div", |props| {
								props.styles(&[Style::SpaceY(24), NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingX(16), Style::PaddingY(24)]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("flow-root")]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Sign in"));
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("flow-root")]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[NoStyle::Noop("-m-2"), Style::Block, Style::Padding(8), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Create account"));
									});
								});
							});
							props.child("11", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("border-t"), NoStyle::Noop("border-gray-200"), Style::PaddingX(16), Style::PaddingY(24)]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[NoStyle::Noop("-m-2"), Style::Flex, Style::ItemsCenter, Style::Padding(8)]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://tailwindui.com/img/flags/flag-canada.svg");
										props.set_attribute("alt", "");
										props.styles(&[Style::Block, NoStyle::Noop("h-auto"), Style::Width(20), NoStyle::Noop("flex-shrink-0")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.styles(&[Style::MarginLeft(12), Style::Block, NoStyle::Noop("text-base"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("CAD"));
									});
									props.child("5", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.set_text(", change currency"));
									});
								});
							});
						});
					});
				});
				props.child("5", Dynamic).run("header", |props| {
					props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("bg-white")]);
					
					props.child("1", Dynamic).run("p", |props| {
						props.styles(&[Style::Flex, Style::Width(40), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("bg-indigo-600"), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-white"), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
						
						props.child("0", Label).run(|props| props.set_text("Get free delivery on orders over $100"));
					});
					props.child("3", Dynamic).run("nav", |props| {
						props.set_attribute("aria-label", "Top");
						props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(16), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("border-b"), NoStyle::Noop("border-gray-200")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::Flex, Style::Width(64), Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("button", |props| {
									props.set_attribute("type", "button");
									props.set_attribute("x-description", "Mobile menu toggle, controls the 'mobileMenuOpen' state.");
									props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::Padding(8), Style::TextColor(Color::Fg(44)), Screen::Large(&[NoStyle::Noop("hidden")])]);
									props.set_attribute("@click", "open = true");
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("-inset-0.5")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.set_text("Open menu"));
									});
									props.child("5", Icon).run(|props| {
										props.style(&[Style::Width(24), Style::Width(24)]);
									});
								});
								//  Logo 
								props.child("5", Dynamic).run("div", |props| {
									props.styles(&[Style::MarginLeft(16), Style::Flex, Screen::Large(&[Style::MarginLeft(0)])]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("sr-only")]);
											
											props.child("0", Label).run(|props| props.set_text("Your Company"));
										});
										props.child("3", Dynamic).run("img", |props| {
											props.styles(&[Style::Width(32), NoStyle::Noop("w-auto")]);
											props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600");
											props.set_attribute("alt", "");
										});
									});
								});
								//  Flyout menus 
								props.child("9", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("hidden"), Screen::Large(&[Style::MarginLeft(32)]), Screen::Large(&[Style::Block]), Screen::Large(&[NoStyle::Noop("self-stretch")])]);
									props.set_attribute("x-data", "Components.popoverGroup()");
									props.set_attribute("x-init", "init()");
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::Flex, NoStyle::Noop("h-full"), Style::SpaceX(32)]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex]);
											props.set_attribute("x-data", "Components.popover({ open: true, focus: false })");
											props.set_attribute("x-init", "init()");
											props.set_attribute("@keydown.escape", "onEscape");
											props.set_attribute("@close-popover-group.window", "onClosePopoverGroup");
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("relative"), Style::Flex]);
												
												props.child("1", Dynamic).run("button", |props| {
													props.set_attribute("type", "button");
													props.set_attribute("x-state:on", "Item active");
													props.set_attribute("x-state:off", "Item inactive");
													props.styles(&[NoStyle::Noop("border-transparent"), Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::TextColor(Color::Fg(89))]), NoStyle::Noop("relative"), NoStyle::Noop("z-10"), NoStyle::Noop("-mb-px"), Style::Flex, Style::ItemsCenter, NoStyle::Noop("border-b-2"), NoStyle::Noop("pt-px"), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("transition-colors"), NoStyle::Noop("duration-200"), NoStyle::Noop("ease-out")]);
													props.set_attribute(":class", "{ 'border-indigo-600 text-indigo-600': open, 'border-transparent text-gray-700 hover:text-gray-800': !(open) }");
													props.set_attribute("@click", "toggle");
													props.set_attribute("@mousedown", "if (open) $event.preventDefault()");
													props.set_attribute("aria-expanded", "false");
													props.set_attribute(":aria-expanded", "open.toString()");
													
													props.child("0", Label).run(|props| props.set_text("Women"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.set_attribute("x-show", "open");
												props.set_attribute("x-transition:enter", "transition ease-out duration-200");
												props.set_attribute("x-transition:enter-start", "opacity-0");
												props.set_attribute("x-transition:enter-end", "opacity-100");
												props.set_attribute("x-transition:leave", "transition ease-in duration-150");
												props.set_attribute("x-transition:leave-start", "opacity-100");
												props.set_attribute("x-transition:leave-end", "opacity-0");
												props.set_attribute("x-description", "'Women' flyout menu, show/hide based on flyout menu state.");
												props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-x-0"), NoStyle::Noop("top-full"), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												props.set_attribute("x-ref", "panel");
												props.set_attribute("@click.away", "open = false");
												
												//  Presentational element used to render the bottom shadow, if we put the shadow on the actual panel it pokes out the top, so we use this shorter element to hide the top of the shadow 
												props.child("3", Dynamic).run("div", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("top-1/2"), NoStyle::Noop("bg-white"), NoStyle::Noop("shadow")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("5", Dynamic).run("div", |props| {
													props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("bg-white")]);
													
													props.child("1", Dynamic).run("div", |props| {
														props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(32)]);
														
														props.child("1", Dynamic).run("div", |props| {
															props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-2"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-10"), Style::PaddingY(64)]);
															
															props.child("1", Dynamic).run("div", |props| {
																props.styles(&[NoStyle::Noop("col-start-2"), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-2"), NoStyle::Noop("gap-x-8")]);
																
																props.child("1", Dynamic).run("div", |props| {
																	props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), NoStyle::Noop("text-base"), Screen::Small(&[NoStyle::Noop("text-sm")])]);
																	
																	props.child("1", Dynamic).run("div", |props| {
																		props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
																		
																		props.child("1", Dynamic).run("img", |props| {
																			props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/mega-menu-category-01.jpg");
																			props.set_attribute("alt", "Models sitting back to back, wearing Basic Tee in black and bone.");
																			props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
																		});
																	});
																	props.child("3", Dynamic).run("a", |props| {
																		props.set_attribute("href", "#");
																		props.styles(&[Style::MarginTop(24), Style::Block, NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("1", Dynamic).run("span", |props| {
																			props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10")]);
																			props.set_attribute("aria-hidden", "true");
																		});
																		props.child("2", Label).run(|props| props.set_text("New Arrivals"));
																	});
																	props.child("5", Dynamic).run("p", |props| {
																		props.set_attribute("aria-hidden", "true");
																		props.styles(&[Style::MarginTop(4)]);
																		
																		props.child("0", Label).run(|props| props.set_text("Shop now"));
																	});
																});
																props.child("3", Dynamic).run("div", |props| {
																	props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), NoStyle::Noop("text-base"), Screen::Small(&[NoStyle::Noop("text-sm")])]);
																	
																	props.child("1", Dynamic).run("div", |props| {
																		props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
																		
																		props.child("1", Dynamic).run("img", |props| {
																			props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/mega-menu-category-02.jpg");
																			props.set_attribute("alt", "Close up of Basic Tee fall bundle with off-white, ochre, olive, and black tees.");
																			props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
																		});
																	});
																	props.child("3", Dynamic).run("a", |props| {
																		props.set_attribute("href", "#");
																		props.styles(&[Style::MarginTop(24), Style::Block, NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("1", Dynamic).run("span", |props| {
																			props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10")]);
																			props.set_attribute("aria-hidden", "true");
																		});
																		props.child("2", Label).run(|props| props.set_text("Basic Tees"));
																	});
																	props.child("5", Dynamic).run("p", |props| {
																		props.set_attribute("aria-hidden", "true");
																		props.styles(&[Style::MarginTop(4)]);
																		
																		props.child("0", Label).run(|props| props.set_text("Shop now"));
																	});
																});
															});
															props.child("3", Dynamic).run("div", |props| {
																props.styles(&[NoStyle::Noop("row-start-1"), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-3"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-10"), NoStyle::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("div", |props| {
																	props.child("1", Dynamic).run("p", |props| {
																		props.set_attribute("id", "Clothing-heading");
																		props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.set_text("Clothing"));
																	});
																	props.child("3", Dynamic).run("ul", |props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Clothing-heading");
																		props.styles(&[Style::MarginTop(24), Style::SpaceY(24), Screen::Small(&[Style::MarginTop(16)]), Screen::Small(&[Style::SpaceY(16)])]);
																		
																		props.child("1", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Tops"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Dresses"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Pants"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Denim"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Sweaters"));
																			});
																		});
																		props.child("11", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("T-Shirts"));
																			});
																		});
																		props.child("13", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Jackets"));
																			});
																		});
																		props.child("15", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Activewear"));
																			});
																		});
																		props.child("17", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Browse All"));
																			});
																		});
																	});
																});
																props.child("3", Dynamic).run("div", |props| {
																	props.child("1", Dynamic).run("p", |props| {
																		props.set_attribute("id", "Accessories-heading");
																		props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.set_text("Accessories"));
																	});
																	props.child("3", Dynamic).run("ul", |props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Accessories-heading");
																		props.styles(&[Style::MarginTop(24), Style::SpaceY(24), Screen::Small(&[Style::MarginTop(16)]), Screen::Small(&[Style::SpaceY(16)])]);
																		
																		props.child("1", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Watches"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Wallets"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Bags"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Sunglasses"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Hats"));
																			});
																		});
																		props.child("11", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Belts"));
																			});
																		});
																	});
																});
																props.child("5", Dynamic).run("div", |props| {
																	props.child("1", Dynamic).run("p", |props| {
																		props.set_attribute("id", "Brands-heading");
																		props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.set_text("Brands"));
																	});
																	props.child("3", Dynamic).run("ul", |props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Brands-heading");
																		props.styles(&[Style::MarginTop(24), Style::SpaceY(24), Screen::Small(&[Style::MarginTop(16)]), Screen::Small(&[Style::SpaceY(16)])]);
																		
																		props.child("1", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Full Nelson"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("My Way"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Re-Arranged"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Counterfeit"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Significant Other"));
																			});
																		});
																	});
																});
															});
														});
													});
												});
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex]);
											props.set_attribute("x-data", "Components.popover({ open: false, focus: false })");
											props.set_attribute("x-init", "init()");
											props.set_attribute("@keydown.escape", "onEscape");
											props.set_attribute("@close-popover-group.window", "onClosePopoverGroup");
											
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("relative"), Style::Flex]);
												
												props.child("1", Dynamic).run("button", |props| {
													props.set_attribute("type", "button");
													props.set_attribute("x-state:on", "Item active");
													props.set_attribute("x-state:off", "Item inactive");
													props.styles(&[NoStyle::Noop("border-transparent"), Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::TextColor(Color::Fg(89))]), NoStyle::Noop("relative"), NoStyle::Noop("z-10"), NoStyle::Noop("-mb-px"), Style::Flex, Style::ItemsCenter, NoStyle::Noop("border-b-2"), NoStyle::Noop("pt-px"), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("transition-colors"), NoStyle::Noop("duration-200"), NoStyle::Noop("ease-out")]);
													props.set_attribute(":class", "{ 'border-indigo-600 text-indigo-600': open, 'border-transparent text-gray-700 hover:text-gray-800': !(open) }");
													props.set_attribute("@click", "toggle");
													props.set_attribute("@mousedown", "if (open) $event.preventDefault()");
													props.set_attribute("aria-expanded", "false");
													props.set_attribute(":aria-expanded", "open.toString()");
													
													props.child("0", Label).run(|props| props.set_text("Men"));
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.set_attribute("x-show", "open");
												props.set_attribute("x-transition:enter", "transition ease-out duration-200");
												props.set_attribute("x-transition:enter-start", "opacity-0");
												props.set_attribute("x-transition:enter-end", "opacity-100");
												props.set_attribute("x-transition:leave", "transition ease-in duration-150");
												props.set_attribute("x-transition:leave-start", "opacity-100");
												props.set_attribute("x-transition:leave-end", "opacity-0");
												props.set_attribute("x-description", "'Men' flyout menu, show/hide based on flyout menu state.");
												props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-x-0"), NoStyle::Noop("top-full"), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												props.set_attribute("x-ref", "panel");
												props.set_attribute("@click.away", "open = false");
												
												//  Presentational element used to render the bottom shadow, if we put the shadow on the actual panel it pokes out the top, so we use this shorter element to hide the top of the shadow 
												props.child("3", Dynamic).run("div", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("top-1/2"), NoStyle::Noop("bg-white"), NoStyle::Noop("shadow")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("5", Dynamic).run("div", |props| {
													props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("bg-white")]);
													
													props.child("1", Dynamic).run("div", |props| {
														props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(32)]);
														
														props.child("1", Dynamic).run("div", |props| {
															props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-2"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-10"), Style::PaddingY(64)]);
															
															props.child("1", Dynamic).run("div", |props| {
																props.styles(&[NoStyle::Noop("col-start-2"), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-2"), NoStyle::Noop("gap-x-8")]);
																
																props.child("1", Dynamic).run("div", |props| {
																	props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), NoStyle::Noop("text-base"), Screen::Small(&[NoStyle::Noop("text-sm")])]);
																	
																	props.child("1", Dynamic).run("div", |props| {
																		props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
																		
																		props.child("1", Dynamic).run("img", |props| {
																			props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-04-detail-product-shot-01.jpg");
																			props.set_attribute("alt", "Drawstring top with elastic loop closure and textured interior padding.");
																			props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
																		});
																	});
																	props.child("3", Dynamic).run("a", |props| {
																		props.set_attribute("href", "#");
																		props.styles(&[Style::MarginTop(24), Style::Block, NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("1", Dynamic).run("span", |props| {
																			props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10")]);
																			props.set_attribute("aria-hidden", "true");
																		});
																		props.child("2", Label).run(|props| props.set_text("New Arrivals"));
																	});
																	props.child("5", Dynamic).run("p", |props| {
																		props.set_attribute("aria-hidden", "true");
																		props.styles(&[Style::MarginTop(4)]);
																		
																		props.child("0", Label).run(|props| props.set_text("Shop now"));
																	});
																});
																props.child("3", Dynamic).run("div", |props| {
																	props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), NoStyle::Noop("text-base"), Screen::Small(&[NoStyle::Noop("text-sm")])]);
																	
																	props.child("1", Dynamic).run("div", |props| {
																		props.styles(&[NoStyle::Noop("aspect-h-1"), NoStyle::Noop("aspect-w-1"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-75"))]);
																		
																		props.child("1", Dynamic).run("img", |props| {
																			props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-02-image-card-06.jpg");
																			props.set_attribute("alt", "Three shirts in gray, white, and blue arranged on table with same line drawing of hands and shapes overlapping on front of shirt.");
																			props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
																		});
																	});
																	props.child("3", Dynamic).run("a", |props| {
																		props.set_attribute("href", "#");
																		props.styles(&[Style::MarginTop(24), Style::Block, NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("1", Dynamic).run("span", |props| {
																			props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10")]);
																			props.set_attribute("aria-hidden", "true");
																		});
																		props.child("2", Label).run(|props| props.set_text("Artwork Tees"));
																	});
																	props.child("5", Dynamic).run("p", |props| {
																		props.set_attribute("aria-hidden", "true");
																		props.styles(&[Style::MarginTop(4)]);
																		
																		props.child("0", Label).run(|props| props.set_text("Shop now"));
																	});
																});
															});
															props.child("3", Dynamic).run("div", |props| {
																props.styles(&[NoStyle::Noop("row-start-1"), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-3"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-10"), NoStyle::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("div", |props| {
																	props.child("1", Dynamic).run("p", |props| {
																		props.set_attribute("id", "Clothing-heading");
																		props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.set_text("Clothing"));
																	});
																	props.child("3", Dynamic).run("ul", |props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Clothing-heading");
																		props.styles(&[Style::MarginTop(24), Style::SpaceY(24), Screen::Small(&[Style::MarginTop(16)]), Screen::Small(&[Style::SpaceY(16)])]);
																		
																		props.child("1", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Tops"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Pants"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Sweaters"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("T-Shirts"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Jackets"));
																			});
																		});
																		props.child("11", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Activewear"));
																			});
																		});
																		props.child("13", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Browse All"));
																			});
																		});
																	});
																});
																props.child("3", Dynamic).run("div", |props| {
																	props.child("1", Dynamic).run("p", |props| {
																		props.set_attribute("id", "Accessories-heading");
																		props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.set_text("Accessories"));
																	});
																	props.child("3", Dynamic).run("ul", |props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Accessories-heading");
																		props.styles(&[Style::MarginTop(24), Style::SpaceY(24), Screen::Small(&[Style::MarginTop(16)]), Screen::Small(&[Style::SpaceY(16)])]);
																		
																		props.child("1", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Watches"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Wallets"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Bags"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Sunglasses"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Hats"));
																			});
																		});
																		props.child("11", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Belts"));
																			});
																		});
																	});
																});
																props.child("5", Dynamic).run("div", |props| {
																	props.child("1", Dynamic).run("p", |props| {
																		props.set_attribute("id", "Brands-heading");
																		props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.set_text("Brands"));
																	});
																	props.child("3", Dynamic).run("ul", |props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Brands-heading");
																		props.styles(&[Style::MarginTop(24), Style::SpaceY(24), Screen::Small(&[Style::MarginTop(16)]), Screen::Small(&[Style::SpaceY(16)])]);
																		
																		props.child("1", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Re-Arranged"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Counterfeit"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("Full Nelson"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.set_text("My Way"));
																			});
																		});
																	});
																});
															});
														});
													});
												});
											});
										});
										props.child("5", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("0", Label).run(|props| props.set_text("Company"));
										});
										props.child("7", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("0", Label).run(|props| props.set_text("Stores"));
										});
									});
								});
								props.child("11", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("ml-auto"), Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("hidden"), Screen::Large(&[Style::Flex]), Screen::Large(&[NoStyle::Noop("flex-1")]), Screen::Large(&[Style::ItemsCenter]), Screen::Large(&[NoStyle::Noop("justify-end")]), Screen::Large(&[Style::SpaceX(24)])]);
										
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("0", Label).run(|props| props.set_text("Sign in"));
										});
										props.child("3", Dynamic).run("span", |props| {
											props.styles(&[Style::Width(24), NoStyle::Noop("w-px"), Style::Color(Color::Fg(22))]);
											props.set_attribute("aria-hidden", "true");
										});
										props.child("5", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("0", Label).run(|props| props.set_text("Create account"));
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("hidden"), Screen::Large(&[Style::MarginLeft(32)]), Screen::Large(&[Style::Flex])]);
										
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::TextColor(Color::Fg(78)), Action::Hover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("1", Dynamic).run("img", |props| {
												props.set_attribute("src", "https://tailwindui.com/img/flags/flag-canada.svg");
												props.set_attribute("alt", "");
												props.styles(&[Style::Block, NoStyle::Noop("h-auto"), Style::Width(20), NoStyle::Noop("flex-shrink-0")]);
											});
											props.child("3", Dynamic).run("span", |props| {
												props.styles(&[Style::MarginLeft(12), Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium")]);
												
												props.child("0", Label).run(|props| props.set_text("CAD"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.set_text(", change currency"));
											});
										});
									});
									//  Search 
									props.child("7", Dynamic).run("div", |props| {
										props.styles(&[Style::Flex, Screen::Large(&[Style::MarginLeft(24)])]);
										
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Padding(8), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))])]);
											
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.set_text("Search"));
											});
											props.child("3", Icon).run(|props| {
												props.style(&[Style::Width(24), Style::Width(24)]);
											});
										});
									});
									//  Cart 
									props.child("11", Dynamic).run("div", |props| {
										props.styles(&[Style::MarginLeft(16), NoStyle::Noop("flow-root"), Screen::Large(&[Style::MarginLeft(24)])]);
										
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("-m-2"), Style::Flex, Style::ItemsCenter, Style::Padding(8)]);
											
											props.child("1", Icon).run(|props| {
												props.style(&[Style::Width(24), Style::Width(24), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44)), NoStyle::NoopGroup("group-hover", Style::TextColor(Color::Fg(56)))]);
											});
											props.child("3", Dynamic).run("span", |props| {
												props.styles(&[Style::MarginLeft(8), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(78)), NoStyle::NoopGroup("group-hover", Style::TextColor(Color::Fg(89)))]);
												
												props.child("0", Label).run(|props| props.set_text("0"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.set_text("items in cart, view bag"));
											});
										});
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
