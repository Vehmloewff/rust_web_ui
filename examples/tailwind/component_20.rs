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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("min-h-[583px]"), Style::Noop("overflow-auto"), Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.set_attribute("x-data", "{ open: false }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.styles(&[Style::Noop("bg-white")]);
				
				//  Mobile menu 
				props.child("3", Dynamic).run("div", |mut props| {
					props.set_attribute("x-show", "open");
					props.styles(&[Style::Noop("relative"), Style::Noop("z-40"), Style::OnScreen(Screen::Large, &[Style::Noop("hidden")])]);
					props.set_attribute("x-description", "Off-canvas menu for mobile, show/hide based on off-canvas menu state.");
					props.set_attribute("x-ref", "dialog");
					props.set_attribute("aria-modal", "true");
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.set_attribute("x-show", "open");
						props.set_attribute("x-transition:enter", "transition-opacity ease-linear duration-300");
						props.set_attribute("x-transition:enter-start", "opacity-0");
						props.set_attribute("x-transition:enter-end", "opacity-100");
						props.set_attribute("x-transition:leave", "transition-opacity ease-linear duration-300");
						props.set_attribute("x-transition:leave-start", "opacity-100");
						props.set_attribute("x-transition:leave-end", "opacity-0");
						props.set_attribute("x-description", "Off-canvas menu backdrop, show/hide based on off-canvas menu state.");
						props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("bg-black"), Style::Noop("bg-opacity-25")]);
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("z-40"), Style::Flex]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.set_attribute("x-show", "open");
							props.set_attribute("x-transition:enter", "transition ease-in-out duration-300 transform");
							props.set_attribute("x-transition:enter-start", "-translate-x-full");
							props.set_attribute("x-transition:enter-end", "translate-x-0");
							props.set_attribute("x-transition:leave", "transition ease-in-out duration-300 transform");
							props.set_attribute("x-transition:leave-start", "translate-x-0");
							props.set_attribute("x-transition:leave-end", "-translate-x-full");
							props.set_attribute("x-description", "Off-canvas menu, show/hide based on off-canvas menu state.");
							props.styles(&[Style::Noop("relative"), Style::Flex, Style::Width(Size::Full), Style::Noop("max-w-xs"), Style::Noop("flex-col"), Style::Noop("overflow-y-auto"), Style::Noop("bg-white"), Style::PaddingBottom(Size::Exact(48)), Style::Noop("shadow-xl")]);
							props.set_attribute("@click.away", "open = false");
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Flex, Style::PaddingX(Size::Exact(16)), Style::PaddingBottom(Size::Exact(8)), Style::PaddingTop(Size::Exact(20))]);
								
								props.child("1", Dynamic).run("button", |mut props| {
									props.set_attribute("type", "button");
									props.styles(&[Style::Noop("relative"), Style::Noop("-m-2"), Style::InlineFlex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(44))]);
									props.set_attribute("@click", "open = false");
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("absolute"), Style::Noop("-inset-0.5")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.text("Close menu"));
									});
									props.child("5", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
									});
								});
							});
							//  Links 
							props.child("5", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8))]);
								props.set_attribute("x-data", "Components.tabs()");
								props.set_attribute("@tab-click.window", "onTabClick");
								props.set_attribute("@tab-keydown.window", "onTabKeydown");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("border-b"), Style::Noop("border-gray-200")]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("-mb-px"), Style::Flex, Style::SpaceX(Size::Exact(32)), Style::PaddingX(Size::Exact(16))]);
										props.set_attribute("aria-orientation", "horizontal");
										props.set_attribute("role", "tablist");
										
										props.child("1", Dynamic).run("button", |mut props| {
											props.set_attribute("id", "tabs-1-tab-1");
											props.styles(&[Style::Noop("border-transparent"), Style::TextColor(Color::Fg(100)), Style::Noop("flex-1"), Style::Noop("whitespace-nowrap"), Style::Noop("border-b-2"), Style::PaddingX(Size::Exact(4)), Style::PaddingY(Size::Exact(16)), Style::Noop("text-base"), Style::Noop("font-medium")]);
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
											
											props.child("0", Label).run(|props| props.text("Women"));
										});
										props.child("3", Dynamic).run("button", |mut props| {
											props.set_attribute("id", "tabs-1-tab-2");
											props.styles(&[Style::Noop("border-transparent"), Style::TextColor(Color::Fg(100)), Style::Noop("flex-1"), Style::Noop("whitespace-nowrap"), Style::Noop("border-b-2"), Style::PaddingX(Size::Exact(4)), Style::PaddingY(Size::Exact(16)), Style::Noop("text-base"), Style::Noop("font-medium")]);
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
											
											props.child("0", Label).run(|props| props.text("Men"));
										});
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.set_attribute("id", "tabs-1-panel-1");
									props.styles(&[Style::SpaceY(Size::Exact(40)), Style::PaddingX(Size::Exact(16)), Style::PaddingBottom(Size::Exact(32)), Style::PaddingTop(Size::Exact(40))]);
									props.set_attribute("x-description", "'Women' tab panel, show/hide based on tab state.");
									props.set_attribute("x-data", "Components.tabPanel(0)");
									props.set_attribute("aria-labelledby", "tabs-1-tab-1");
									props.set_attribute("x-init", "init()");
									props.set_attribute("x-show", "selected");
									props.set_attribute("@tab-select.window", "onTabSelect");
									props.set_attribute("role", "tabpanel");
									props.set_attribute("tabindex", "0");
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-2"), Style::Noop("gap-x-4")]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Noop("text-sm")]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/mega-menu-category-01.jpg");
													props.set_attribute("alt", "Models sitting back to back, wearing Basic Tee in black and bone.");
													props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Block, Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("z-10")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("2", Label).run(|props| props.text("New Arrivals"));
											});
											props.child("5", Dynamic).run("p", |mut props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::MarginTop(Size::Exact(4))]);
												
												props.child("0", Label).run(|props| props.text("Shop now"));
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Noop("text-sm")]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/mega-menu-category-02.jpg");
													props.set_attribute("alt", "Close up of Basic Tee fall bundle with off-white, ochre, olive, and black tees.");
													props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Block, Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("z-10")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("2", Label).run(|props| props.text("Basic Tees"));
											});
											props.child("5", Dynamic).run("p", |mut props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::MarginTop(Size::Exact(4))]);
												
												props.child("0", Label).run(|props| props.text("Shop now"));
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("p", |mut props| {
											props.set_attribute("id", "women-clothing-heading-mobile");
											props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Clothing"));
										});
										props.child("3", Dynamic).run("ul", |mut props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "women-clothing-heading-mobile");
											props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::Noop("flex-col"), Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Tops"));
												});
											});
											props.child("3", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Dresses"));
												});
											});
											props.child("5", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Pants"));
												});
											});
											props.child("7", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Denim"));
												});
											});
											props.child("9", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Sweaters"));
												});
											});
											props.child("11", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("T-Shirts"));
												});
											});
											props.child("13", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Jackets"));
												});
											});
											props.child("15", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Activewear"));
												});
											});
											props.child("17", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Browse All"));
												});
											});
										});
									});
									props.child("5", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("p", |mut props| {
											props.set_attribute("id", "women-accessories-heading-mobile");
											props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Accessories"));
										});
										props.child("3", Dynamic).run("ul", |mut props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "women-accessories-heading-mobile");
											props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::Noop("flex-col"), Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Watches"));
												});
											});
											props.child("3", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Wallets"));
												});
											});
											props.child("5", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Bags"));
												});
											});
											props.child("7", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Sunglasses"));
												});
											});
											props.child("9", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Hats"));
												});
											});
											props.child("11", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Belts"));
												});
											});
										});
									});
									props.child("7", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("p", |mut props| {
											props.set_attribute("id", "women-brands-heading-mobile");
											props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Brands"));
										});
										props.child("3", Dynamic).run("ul", |mut props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "women-brands-heading-mobile");
											props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::Noop("flex-col"), Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Full Nelson"));
												});
											});
											props.child("3", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("My Way"));
												});
											});
											props.child("5", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Re-Arranged"));
												});
											});
											props.child("7", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Counterfeit"));
												});
											});
											props.child("9", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Significant Other"));
												});
											});
										});
									});
								});
								props.child("5", Dynamic).run("div", |mut props| {
									props.set_attribute("id", "tabs-1-panel-2");
									props.styles(&[Style::SpaceY(Size::Exact(40)), Style::PaddingX(Size::Exact(16)), Style::PaddingBottom(Size::Exact(32)), Style::PaddingTop(Size::Exact(40))]);
									props.set_attribute("x-description", "'Men' tab panel, show/hide based on tab state.");
									props.set_attribute("x-data", "Components.tabPanel(0)");
									props.set_attribute("aria-labelledby", "tabs-1-tab-2");
									props.set_attribute("x-init", "init()");
									props.set_attribute("x-show", "selected");
									props.set_attribute("@tab-select.window", "onTabSelect");
									props.set_attribute("role", "tabpanel");
									props.set_attribute("tabindex", "0");
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-2"), Style::Noop("gap-x-4")]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Noop("text-sm")]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-04-detail-product-shot-01.jpg");
													props.set_attribute("alt", "Drawstring top with elastic loop closure and textured interior padding.");
													props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Block, Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("z-10")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("2", Label).run(|props| props.text("New Arrivals"));
											});
											props.child("5", Dynamic).run("p", |mut props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::MarginTop(Size::Exact(4))]);
												
												props.child("0", Label).run(|props| props.text("Shop now"));
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Noop("text-sm")]);
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
												
												props.child("1", Dynamic).run("img", |mut props| {
													props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-02-image-card-06.jpg");
													props.set_attribute("alt", "Three shirts in gray, white, and blue arranged on table with same line drawing of hands and shapes overlapping on front of shirt.");
													props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
												});
											});
											props.child("3", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Block, Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
												
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("z-10")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("2", Label).run(|props| props.text("Artwork Tees"));
											});
											props.child("5", Dynamic).run("p", |mut props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::MarginTop(Size::Exact(4))]);
												
												props.child("0", Label).run(|props| props.text("Shop now"));
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("p", |mut props| {
											props.set_attribute("id", "men-clothing-heading-mobile");
											props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Clothing"));
										});
										props.child("3", Dynamic).run("ul", |mut props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "men-clothing-heading-mobile");
											props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::Noop("flex-col"), Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Tops"));
												});
											});
											props.child("3", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Pants"));
												});
											});
											props.child("5", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Sweaters"));
												});
											});
											props.child("7", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("T-Shirts"));
												});
											});
											props.child("9", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Jackets"));
												});
											});
											props.child("11", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Activewear"));
												});
											});
											props.child("13", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Browse All"));
												});
											});
										});
									});
									props.child("5", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("p", |mut props| {
											props.set_attribute("id", "men-accessories-heading-mobile");
											props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Accessories"));
										});
										props.child("3", Dynamic).run("ul", |mut props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "men-accessories-heading-mobile");
											props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::Noop("flex-col"), Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Watches"));
												});
											});
											props.child("3", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Wallets"));
												});
											});
											props.child("5", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Bags"));
												});
											});
											props.child("7", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Sunglasses"));
												});
											});
											props.child("9", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Hats"));
												});
											});
											props.child("11", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Belts"));
												});
											});
										});
									});
									props.child("7", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("p", |mut props| {
											props.set_attribute("id", "men-brands-heading-mobile");
											props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Brands"));
										});
										props.child("3", Dynamic).run("ul", |mut props| {
											props.set_attribute("role", "list");
											props.set_attribute("aria-labelledby", "men-brands-heading-mobile");
											props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::Noop("flex-col"), Style::SpaceY(Size::Exact(24))]);
											
											props.child("1", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Re-Arranged"));
												});
											});
											props.child("3", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Counterfeit"));
												});
											});
											props.child("5", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("Full Nelson"));
												});
											});
											props.child("7", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Noop("flow-root")]);
												
												props.child("1", Dynamic).run("a", |mut props| {
													props.set_attribute("href", "#");
													props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(56))]);
													
													props.child("0", Label).run(|props| props.text("My Way"));
												});
											});
										});
									});
								});
							});
							props.child("7", Dynamic).run("div", |mut props| {
								props.styles(&[Style::SpaceY(Size::Exact(24)), Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24))]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("flow-root")]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Company"));
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("flow-root")]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Stores"));
									});
								});
							});
							props.child("9", Dynamic).run("div", |mut props| {
								props.styles(&[Style::SpaceY(Size::Exact(24)), Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24))]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("flow-root")]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Sign in"));
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("flow-root")]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Noop("-m-2"), Style::Block, Style::Padding(Size::Exact(8)), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Create account"));
									});
								});
							});
							props.child("11", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("border-t"), Style::Noop("border-gray-200"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24))]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Noop("-m-2"), Style::Flex, Style::ItemsCenter, Style::Padding(Size::Exact(8))]);
									
									props.child("1", Dynamic).run("img", |mut props| {
										props.set_attribute("src", "https://tailwindui.com/img/flags/flag-canada.svg");
										props.set_attribute("alt", "");
										props.styles(&[Style::Block, Style::Noop("h-auto"), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("text-base"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("CAD"));
									});
									props.child("5", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.text(", change currency"));
									});
								});
							});
						});
					});
				});
				props.child("5", Dynamic).run("header", |mut props| {
					props.styles(&[Style::Noop("relative"), Style::Noop("bg-white")]);
					
					props.child("1", Dynamic).run("p", |mut props| {
						props.styles(&[Style::Flex, Style::Width(Size::Exact(40)), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("text-white"), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
						
						props.child("0", Label).run(|props| props.text("Get free delivery on orders over $100"));
					});
					props.child("3", Dynamic).run("nav", |mut props| {
						props.set_attribute("aria-label", "Top");
						props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("border-b"), Style::Noop("border-gray-200")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Flex, Style::Width(Size::Exact(64)), Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("button", |mut props| {
									props.set_attribute("type", "button");
									props.set_attribute("x-description", "Mobile menu toggle, controls the 'mobileMenuOpen' state.");
									props.styles(&[Style::Noop("relative"), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(44)), Style::OnScreen(Screen::Large, &[Style::Noop("hidden")])]);
									props.set_attribute("@click", "open = true");
									
									props.child("1", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("absolute"), Style::Noop("-inset-0.5")]);
									});
									props.child("3", Dynamic).run("span", |mut props| {
										props.styles(&[Style::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.text("Open menu"));
									});
									props.child("5", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
									});
								});
								//  Logo 
								props.child("5", Dynamic).run("div", |mut props| {
									props.styles(&[Style::MarginLeft(Size::Exact(16)), Style::Flex, Style::OnScreen(Screen::Large, &[Style::MarginLeft(Size::Exact(0))])]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("sr-only")]);
											
											props.child("0", Label).run(|props| props.text("Your Company"));
										});
										props.child("3", Dynamic).run("img", |mut props| {
											props.styles(&[Style::Width(Size::Exact(32)), Style::Noop("w-auto")]);
											props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600");
											props.set_attribute("alt", "");
										});
									});
								});
								//  Flyout menus 
								props.child("9", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Large, &[Style::MarginLeft(Size::Exact(32))]), Style::OnScreen(Screen::Large, &[Style::Block]), Style::OnScreen(Screen::Large, &[Style::Noop("self-stretch")])]);
									props.set_attribute("x-data", "Components.popoverGroup()");
									props.set_attribute("x-init", "init()");
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Flex, Style::Width(Size::Full), Style::SpaceX(Size::Exact(32))]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex]);
											props.set_attribute("x-data", "Components.popover({ open: true, focus: false })");
											props.set_attribute("x-init", "init()");
											props.set_attribute("@keydown.escape", "onEscape");
											props.set_attribute("@close-popover-group.window", "onClosePopoverGroup");
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("relative"), Style::Flex]);
												
												props.child("1", Dynamic).run("button", |mut props| {
													props.set_attribute("type", "button");
													props.set_attribute("x-state:on", "Item active");
													props.set_attribute("x-state:off", "Item inactive");
													props.styles(&[Style::Noop("border-transparent"), Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::TextColor(Color::Fg(89))]), Style::Noop("relative"), Style::Noop("z-10"), Style::Noop("-mb-px"), Style::Flex, Style::ItemsCenter, Style::Noop("border-b-2"), Style::Noop("pt-px"), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("transition-colors"), Style::Noop("duration-200"), Style::Noop("ease-out")]);
													props.set_attribute(":class", "{ 'border-indigo-600 text-indigo-600': open, 'border-transparent text-gray-700 hover:text-gray-800': !(open) }");
													props.set_attribute("@click", "toggle");
													props.set_attribute("@mousedown", "if (open) $event.preventDefault()");
													props.set_attribute("aria-expanded", "false");
													props.set_attribute(":aria-expanded", "open.toString()");
													
													props.child("0", Label).run(|props| props.text("Women"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.set_attribute("x-show", "open");
												props.set_attribute("x-transition:enter", "transition ease-out duration-200");
												props.set_attribute("x-transition:enter-start", "opacity-0");
												props.set_attribute("x-transition:enter-end", "opacity-100");
												props.set_attribute("x-transition:leave", "transition ease-in duration-150");
												props.set_attribute("x-transition:leave-start", "opacity-100");
												props.set_attribute("x-transition:leave-end", "opacity-0");
												props.set_attribute("x-description", "'Women' flyout menu, show/hide based on flyout menu state.");
												props.styles(&[Style::Noop("absolute"), Style::Noop("inset-x-0"), Style::Noop("top-full"), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												props.set_attribute("x-ref", "panel");
												props.set_attribute("@click.away", "open = false");
												
												//  Presentational element used to render the bottom shadow, if we put the shadow on the actual panel it pokes out the top, so we use this shorter element to hide the top of the shadow 
												props.child("3", Dynamic).run("div", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("top-1/2"), Style::Noop("bg-white"), Style::Noop("shadow")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("5", Dynamic).run("div", |mut props| {
													props.styles(&[Style::Noop("relative"), Style::Noop("bg-white")]);
													
													props.child("1", Dynamic).run("div", |mut props| {
														props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(32))]);
														
														props.child("1", Dynamic).run("div", |mut props| {
															props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-2"), Style::Noop("gap-x-8"), Style::Noop("gap-y-10"), Style::PaddingY(Size::Exact(64))]);
															
															props.child("1", Dynamic).run("div", |mut props| {
																props.styles(&[Style::Noop("col-start-2"), Style::Noop("grid"), Style::Noop("grid-cols-2"), Style::Noop("gap-x-8")]);
																
																props.child("1", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Noop("text-base"), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")])]);
																	
																	props.child("1", Dynamic).run("div", |mut props| {
																		props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
																		
																		props.child("1", Dynamic).run("img", |mut props| {
																			props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/mega-menu-category-01.jpg");
																			props.set_attribute("alt", "Models sitting back to back, wearing Basic Tee in black and bone.");
																			props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
																		});
																	});
																	props.child("3", Dynamic).run("a", |mut props| {
																		props.set_attribute("href", "#");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Block, Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("1", Dynamic).run("span", |mut props| {
																			props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("z-10")]);
																			props.set_attribute("aria-hidden", "true");
																		});
																		props.child("2", Label).run(|props| props.text("New Arrivals"));
																	});
																	props.child("5", Dynamic).run("p", |mut props| {
																		props.set_attribute("aria-hidden", "true");
																		props.styles(&[Style::MarginTop(Size::Exact(4))]);
																		
																		props.child("0", Label).run(|props| props.text("Shop now"));
																	});
																});
																props.child("3", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Noop("text-base"), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")])]);
																	
																	props.child("1", Dynamic).run("div", |mut props| {
																		props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
																		
																		props.child("1", Dynamic).run("img", |mut props| {
																			props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/mega-menu-category-02.jpg");
																			props.set_attribute("alt", "Close up of Basic Tee fall bundle with off-white, ochre, olive, and black tees.");
																			props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
																		});
																	});
																	props.child("3", Dynamic).run("a", |mut props| {
																		props.set_attribute("href", "#");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Block, Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("1", Dynamic).run("span", |mut props| {
																			props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("z-10")]);
																			props.set_attribute("aria-hidden", "true");
																		});
																		props.child("2", Label).run(|props| props.text("Basic Tees"));
																	});
																	props.child("5", Dynamic).run("p", |mut props| {
																		props.set_attribute("aria-hidden", "true");
																		props.styles(&[Style::MarginTop(Size::Exact(4))]);
																		
																		props.child("0", Label).run(|props| props.text("Shop now"));
																	});
																});
															});
															props.child("3", Dynamic).run("div", |mut props| {
																props.styles(&[Style::Noop("row-start-1"), Style::Noop("grid"), Style::Noop("grid-cols-3"), Style::Noop("gap-x-8"), Style::Noop("gap-y-10"), Style::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("div", |mut props| {
																	props.child("1", Dynamic).run("p", |mut props| {
																		props.set_attribute("id", "Clothing-heading");
																		props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.text("Clothing"));
																	});
																	props.child("3", Dynamic).run("ul", |mut props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Clothing-heading");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::SpaceY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(16))]), Style::OnScreen(Screen::Small, &[Style::SpaceY(Size::Exact(16))])]);
																		
																		props.child("1", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Tops"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Dresses"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Pants"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Denim"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Sweaters"));
																			});
																		});
																		props.child("11", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("T-Shirts"));
																			});
																		});
																		props.child("13", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Jackets"));
																			});
																		});
																		props.child("15", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Activewear"));
																			});
																		});
																		props.child("17", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Browse All"));
																			});
																		});
																	});
																});
																props.child("3", Dynamic).run("div", |mut props| {
																	props.child("1", Dynamic).run("p", |mut props| {
																		props.set_attribute("id", "Accessories-heading");
																		props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.text("Accessories"));
																	});
																	props.child("3", Dynamic).run("ul", |mut props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Accessories-heading");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::SpaceY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(16))]), Style::OnScreen(Screen::Small, &[Style::SpaceY(Size::Exact(16))])]);
																		
																		props.child("1", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Watches"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Wallets"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Bags"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Sunglasses"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Hats"));
																			});
																		});
																		props.child("11", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Belts"));
																			});
																		});
																	});
																});
																props.child("5", Dynamic).run("div", |mut props| {
																	props.child("1", Dynamic).run("p", |mut props| {
																		props.set_attribute("id", "Brands-heading");
																		props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.text("Brands"));
																	});
																	props.child("3", Dynamic).run("ul", |mut props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Brands-heading");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::SpaceY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(16))]), Style::OnScreen(Screen::Small, &[Style::SpaceY(Size::Exact(16))])]);
																		
																		props.child("1", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Full Nelson"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("My Way"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Re-Arranged"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Counterfeit"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Significant Other"));
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
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex]);
											props.set_attribute("x-data", "Components.popover({ open: false, focus: false })");
											props.set_attribute("x-init", "init()");
											props.set_attribute("@keydown.escape", "onEscape");
											props.set_attribute("@close-popover-group.window", "onClosePopoverGroup");
											
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("relative"), Style::Flex]);
												
												props.child("1", Dynamic).run("button", |mut props| {
													props.set_attribute("type", "button");
													props.set_attribute("x-state:on", "Item active");
													props.set_attribute("x-state:off", "Item inactive");
													props.styles(&[Style::Noop("border-transparent"), Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::TextColor(Color::Fg(89))]), Style::Noop("relative"), Style::Noop("z-10"), Style::Noop("-mb-px"), Style::Flex, Style::ItemsCenter, Style::Noop("border-b-2"), Style::Noop("pt-px"), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("transition-colors"), Style::Noop("duration-200"), Style::Noop("ease-out")]);
													props.set_attribute(":class", "{ 'border-indigo-600 text-indigo-600': open, 'border-transparent text-gray-700 hover:text-gray-800': !(open) }");
													props.set_attribute("@click", "toggle");
													props.set_attribute("@mousedown", "if (open) $event.preventDefault()");
													props.set_attribute("aria-expanded", "false");
													props.set_attribute(":aria-expanded", "open.toString()");
													
													props.child("0", Label).run(|props| props.text("Men"));
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.set_attribute("x-show", "open");
												props.set_attribute("x-transition:enter", "transition ease-out duration-200");
												props.set_attribute("x-transition:enter-start", "opacity-0");
												props.set_attribute("x-transition:enter-end", "opacity-100");
												props.set_attribute("x-transition:leave", "transition ease-in duration-150");
												props.set_attribute("x-transition:leave-start", "opacity-100");
												props.set_attribute("x-transition:leave-end", "opacity-0");
												props.set_attribute("x-description", "'Men' flyout menu, show/hide based on flyout menu state.");
												props.styles(&[Style::Noop("absolute"), Style::Noop("inset-x-0"), Style::Noop("top-full"), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												props.set_attribute("x-ref", "panel");
												props.set_attribute("@click.away", "open = false");
												
												//  Presentational element used to render the bottom shadow, if we put the shadow on the actual panel it pokes out the top, so we use this shorter element to hide the top of the shadow 
												props.child("3", Dynamic).run("div", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("top-1/2"), Style::Noop("bg-white"), Style::Noop("shadow")]);
													props.set_attribute("aria-hidden", "true");
												});
												props.child("5", Dynamic).run("div", |mut props| {
													props.styles(&[Style::Noop("relative"), Style::Noop("bg-white")]);
													
													props.child("1", Dynamic).run("div", |mut props| {
														props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(32))]);
														
														props.child("1", Dynamic).run("div", |mut props| {
															props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-2"), Style::Noop("gap-x-8"), Style::Noop("gap-y-10"), Style::PaddingY(Size::Exact(64))]);
															
															props.child("1", Dynamic).run("div", |mut props| {
																props.styles(&[Style::Noop("col-start-2"), Style::Noop("grid"), Style::Noop("grid-cols-2"), Style::Noop("gap-x-8")]);
																
																props.child("1", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Noop("text-base"), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")])]);
																	
																	props.child("1", Dynamic).run("div", |mut props| {
																		props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
																		
																		props.child("1", Dynamic).run("img", |mut props| {
																			props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-04-detail-product-shot-01.jpg");
																			props.set_attribute("alt", "Drawstring top with elastic loop closure and textured interior padding.");
																			props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
																		});
																	});
																	props.child("3", Dynamic).run("a", |mut props| {
																		props.set_attribute("href", "#");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Block, Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("1", Dynamic).run("span", |mut props| {
																			props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("z-10")]);
																			props.set_attribute("aria-hidden", "true");
																		});
																		props.child("2", Label).run(|props| props.text("New Arrivals"));
																	});
																	props.child("5", Dynamic).run("p", |mut props| {
																		props.set_attribute("aria-hidden", "true");
																		props.styles(&[Style::MarginTop(Size::Exact(4))]);
																		
																		props.child("0", Label).run(|props| props.text("Shop now"));
																	});
																});
																props.child("3", Dynamic).run("div", |mut props| {
																	props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Noop("text-base"), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")])]);
																	
																	props.child("1", Dynamic).run("div", |mut props| {
																		props.styles(&[Style::Noop("aspect-h-1"), Style::Noop("aspect-w-1"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Style::NoopGroup("group-hover", &[Style::Noop("opacity-75")])]);
																		
																		props.child("1", Dynamic).run("img", |mut props| {
																			props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/category-page-02-image-card-06.jpg");
																			props.set_attribute("alt", "Three shirts in gray, white, and blue arranged on table with same line drawing of hands and shapes overlapping on front of shirt.");
																			props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
																		});
																	});
																	props.child("3", Dynamic).run("a", |mut props| {
																		props.set_attribute("href", "#");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Block, Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("1", Dynamic).run("span", |mut props| {
																			props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("z-10")]);
																			props.set_attribute("aria-hidden", "true");
																		});
																		props.child("2", Label).run(|props| props.text("Artwork Tees"));
																	});
																	props.child("5", Dynamic).run("p", |mut props| {
																		props.set_attribute("aria-hidden", "true");
																		props.styles(&[Style::MarginTop(Size::Exact(4))]);
																		
																		props.child("0", Label).run(|props| props.text("Shop now"));
																	});
																});
															});
															props.child("3", Dynamic).run("div", |mut props| {
																props.styles(&[Style::Noop("row-start-1"), Style::Noop("grid"), Style::Noop("grid-cols-3"), Style::Noop("gap-x-8"), Style::Noop("gap-y-10"), Style::Noop("text-sm")]);
																
																props.child("1", Dynamic).run("div", |mut props| {
																	props.child("1", Dynamic).run("p", |mut props| {
																		props.set_attribute("id", "Clothing-heading");
																		props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.text("Clothing"));
																	});
																	props.child("3", Dynamic).run("ul", |mut props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Clothing-heading");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::SpaceY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(16))]), Style::OnScreen(Screen::Small, &[Style::SpaceY(Size::Exact(16))])]);
																		
																		props.child("1", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Tops"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Pants"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Sweaters"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("T-Shirts"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Jackets"));
																			});
																		});
																		props.child("11", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Activewear"));
																			});
																		});
																		props.child("13", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Browse All"));
																			});
																		});
																	});
																});
																props.child("3", Dynamic).run("div", |mut props| {
																	props.child("1", Dynamic).run("p", |mut props| {
																		props.set_attribute("id", "Accessories-heading");
																		props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.text("Accessories"));
																	});
																	props.child("3", Dynamic).run("ul", |mut props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Accessories-heading");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::SpaceY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(16))]), Style::OnScreen(Screen::Small, &[Style::SpaceY(Size::Exact(16))])]);
																		
																		props.child("1", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Watches"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Wallets"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Bags"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Sunglasses"));
																			});
																		});
																		props.child("9", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Hats"));
																			});
																		});
																		props.child("11", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Belts"));
																			});
																		});
																	});
																});
																props.child("5", Dynamic).run("div", |mut props| {
																	props.child("1", Dynamic).run("p", |mut props| {
																		props.set_attribute("id", "Brands-heading");
																		props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
																		
																		props.child("0", Label).run(|props| props.text("Brands"));
																	});
																	props.child("3", Dynamic).run("ul", |mut props| {
																		props.set_attribute("role", "list");
																		props.set_attribute("aria-labelledby", "Brands-heading");
																		props.styles(&[Style::MarginTop(Size::Exact(24)), Style::SpaceY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(16))]), Style::OnScreen(Screen::Small, &[Style::SpaceY(Size::Exact(16))])]);
																		
																		props.child("1", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Re-Arranged"));
																			});
																		});
																		props.child("3", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Counterfeit"));
																			});
																		});
																		props.child("5", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("Full Nelson"));
																			});
																		});
																		props.child("7", Dynamic).run("li", |mut props| {
																			props.styles(&[Style::Flex]);
																			
																			props.child("1", Dynamic).run("a", |mut props| {
																				props.set_attribute("href", "#");
																				props.styles(&[Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
																				
																				props.child("0", Label).run(|props| props.text("My Way"));
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
										props.child("5", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("0", Label).run(|props| props.text("Company"));
										});
										props.child("7", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("0", Label).run(|props| props.text("Stores"));
										});
									});
								});
								props.child("11", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("ml-auto"), Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Large, &[Style::Flex]), Style::OnScreen(Screen::Large, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Large, &[Style::ItemsCenter]), Style::OnScreen(Screen::Large, &[Style::Noop("justify-end")]), Style::OnScreen(Screen::Large, &[Style::SpaceX(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("0", Label).run(|props| props.text("Sign in"));
										});
										props.child("3", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Width(Size::Exact(24)), Style::Noop("w-px"), Style::Color(Color::Fg(22))]);
											props.set_attribute("aria-hidden", "true");
										});
										props.child("5", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("0", Label).run(|props| props.text("Create account"));
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Large, &[Style::MarginLeft(Size::Exact(32))]), Style::OnScreen(Screen::Large, &[Style::Flex])]);
										
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Flex, Style::ItemsCenter, Style::TextColor(Color::Fg(78)), Style::OnHover(&[Style::TextColor(Color::Fg(89))])]);
											
											props.child("1", Dynamic).run("img", |mut props| {
												props.set_attribute("src", "https://tailwindui.com/img/flags/flag-canada.svg");
												props.set_attribute("alt", "");
												props.styles(&[Style::Block, Style::Noop("h-auto"), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0")]);
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium")]);
												
												props.child("0", Label).run(|props| props.text("CAD"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.text(", change currency"));
											});
										});
									});
									//  Search 
									props.child("7", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Flex, Style::OnScreen(Screen::Large, &[Style::MarginLeft(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))])]);
											
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.text("Search"));
											});
											props.child("3", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
											});
										});
									});
									//  Cart 
									props.child("11", Dynamic).run("div", |mut props| {
										props.styles(&[Style::MarginLeft(Size::Exact(16)), Style::Noop("flow-root"), Style::OnScreen(Screen::Large, &[Style::MarginLeft(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::Noop("group"), Style::Noop("-m-2"), Style::Flex, Style::ItemsCenter, Style::Padding(Size::Exact(8))]);
											
											props.child("1", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44)), Style::NoopGroup("group-hover", &[Style::TextColor(Color::Fg(56))])]);
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.styles(&[Style::MarginLeft(Size::Exact(8)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(78)), Style::NoopGroup("group-hover", &[Style::TextColor(Color::Fg(89))])]);
												
												props.child("0", Label).run(|props| props.text("0"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.text("items in cart, view bag"));
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
